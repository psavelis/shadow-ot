#!/usr/bin/env bash
set -euo pipefail

CLUSTER_NAME="shadow-e2e"

detect_arch() {
  local os=$(uname -s | tr '[:upper:]' '[:lower:]')
  local arch=$(uname -m)
  case "$os" in
    darwin)
      if [[ "$arch" == "arm64" ]]; then echo "darwin-arm64"; else echo "darwin-amd64"; fi
      ;;
    linux)
      if [[ "$arch" == "aarch64" || "$arch" == "arm64" ]]; then echo "linux-arm64"; else echo "linux-amd64"; fi
      ;;
    *) echo "Unsupported OS: $os" && exit 1;;
  esac
}

install_kind() {
  if ! command -v kind >/dev/null 2>&1; then
    local target=$(detect_arch)
    echo "Installing kind for $target"
    curl -Lo ./kind "https://kind.sigs.k8s.io/dl/v0.23.0/kind-$target"
    chmod +x ./kind
    mkdir -p "$(pwd)/scripts/bin"
    mv ./kind "$(pwd)/scripts/bin/kind"
    export PATH="$(pwd)/scripts/bin:$PATH"
    echo "kind installed to scripts/bin"
  fi
}

install_kubectl() {
  if ! command -v kubectl >/dev/null 2>&1; then
    echo "kubectl is required"
    exit 1
  fi
}

create_cluster() {
  if ! kind get clusters | grep -q "^${CLUSTER_NAME}$"; then
    kind create cluster --name "$CLUSTER_NAME"
  fi
}

setup_metallb() {
  kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.10/config/manifests/metallb-native.yaml
  kubectl -n metallb-system rollout status deploy/controller --timeout=120s || true
  # wait a bit for webhook to be ready
  sleep 5
  # Determine docker network CIDR
  local cidr
  cidr=$(docker network inspect kind -f '{{ range .IPAM.Config }}{{ .Subnet }} {{ end }}' | tr ' ' '\n' | grep -E '^[0-9]+\.' | head -n1)
  if [[ -z "$cidr" ]]; then echo "Failed to determine kind network CIDR" && exit 1; fi
  # Derive a small pool at the end of the subnet
  local base=$(echo "$cidr" | cut -d'/' -f1 | awk -F. '{print $1"."$2"."$3}')
  cat <<EOF | kubectl apply -f -
apiVersion: metallb.io/v1beta1
kind: IPAddressPool
metadata:
  name: kind-pool
  namespace: metallb-system
spec:
  addresses:
    - ${base}.240-${base}.250
---
apiVersion: metallb.io/v1beta1
kind: L2Advertisement
metadata:
  name: kind-advert
  namespace: metallb-system
spec: {}
EOF
}

deploy_stack() {
  kubectl apply -k k8s/base
  kubectl apply -f k8s/base/secrets-example.yaml
  kubectl apply -k k8s/overlays/dev
}

build_and_load_images() {
  echo "Building local images and loading into kind..."
  docker build -f docker/Dockerfile.server -t shadow-ot/server:local . || true
  docker build -f web/landing/Dockerfile -t shadow-ot/web:local web/landing
  docker build -f web/admin/Dockerfile -t shadow-ot/admin:local web/admin
  kind load docker-image shadow-ot/server:local --name "$CLUSTER_NAME" || true
  kind load docker-image shadow-ot/web:local --name "$CLUSTER_NAME"
  kind load docker-image shadow-ot/admin:local --name "$CLUSTER_NAME"
  kubectl -n shadow-ot set image deploy/shadow-server shadow-server=shadow-ot/server:local || true
  kubectl -n shadow-ot set image deploy/shadow-web shadow-web=shadow-ot/web:local || true
  kubectl -n shadow-ot set image deploy/shadow-admin shadow-admin=shadow-ot/admin:local || true
}

wait_ready() {
  kubectl wait --for=condition=available deploy/shadow-server -n shadow-ot --timeout=300s || true
  kubectl wait --for=condition=available deploy/shadow-web -n shadow-ot --timeout=300s || true
  kubectl wait --for=condition=available deploy/shadow-admin -n shadow-ot --timeout=300s || true
  kubectl wait --for=condition=available deploy/shadow-download -n shadow-ot --timeout=300s
}

show_ips() {
  kubectl get svc -n shadow-ot -o wide
  WEB_IP=$(kubectl get svc shadow-web-external -n shadow-ot -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
  ADMIN_IP=$(kubectl get svc shadow-admin-external -n shadow-ot -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
  GAME_IP=$(kubectl get svc shadow-server-external -n shadow-ot -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
  DL_IP=$(kubectl get svc shadow-download -n shadow-ot -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
  echo "WEB_IP=$WEB_IP ADMIN_IP=$ADMIN_IP GAME_IP=$GAME_IP DL_IP=$DL_IP"
}

smoke_tests() {
  if [[ -n "$DL_IP" ]]; then curl -fsS --max-time 10 http://$DL_IP/ || true; fi
  if [[ -n "$WEB_IP" ]]; then curl -fsS --max-time 10 http://$WEB_IP/ || true; fi
  API_IP=$(kubectl get svc shadow-server -n shadow-ot -o jsonpath='{.spec.clusterIP}')
  kubectl run tmp --rm -it --image=curlimages/curl -n shadow-ot --restart=Never -- curl -fsS http://$API_IP:8080/health || true
}

install_kind
install_kubectl
create_cluster
setup_metallb
deploy_stack
build_and_load_images
wait_ready
show_ips
smoke_tests
