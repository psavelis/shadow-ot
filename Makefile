SHELL := /bin/bash

# Local cluster and namespace
CLUSTER ?= shadow-local
NAMESPACE ?= shadow-ot

.PHONY: up down kind metallb build-images load-images deploy health user-flow ip

up: kind metallb build-images load-images deploy health user-flow ip

kind:
	@if ! kind get clusters | grep -q $(CLUSTER); then \
		kind create cluster --name $(CLUSTER); \
	fi

metallb:
	kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.10/config/manifests/metallb-native.yaml
	@cat <<EOF | kubectl apply -f -
	apiVersion: metallb.io/v1beta1
	kind: IPAddressPool
	metadata:
	  name: kind-pool
	  namespace: metallb-system
	spec:
	  addresses:
	    - 172.18.0.240-172.18.0.250
	---
	apiVersion: metallb.io/v1beta1
	kind: L2Advertisement
	metadata:
	  name: kind-advert
	  namespace: metallb-system
	spec: {}
	EOF

build-images:
	docker buildx build -f docker/Dockerfile.server -t shadow-ot/server:local --load .
	docker buildx build -f web/landing/Dockerfile -t shadow-ot/web:local --load web/landing
	docker buildx build -f web/admin/Dockerfile -t shadow-ot/admin:local --load web/admin

load-images:
	kind load docker-image shadow-ot/server:local --name $(CLUSTER)
	kind load docker-image shadow-ot/web:local --name $(CLUSTER)
	kind load docker-image shadow-ot/admin:local --name $(CLUSTER)

deploy:
	kubectl apply -f k8s/base/secrets-example.yaml
	kubectl apply -k k8s/base
	kubectl apply -k k8s/overlays/dev
	kubectl set image deploy/shadow-server shadow-server=shadow-ot/server:local -n $(NAMESPACE)
	kubectl set image deploy/shadow-web shadow-web=shadow-ot/web:local -n $(NAMESPACE)
	kubectl set image deploy/shadow-admin shadow-admin=shadow-ot/admin:local -n $(NAMESPACE)

health:
	kubectl wait --for=condition=available deploy/shadow-server -n $(NAMESPACE) --timeout=300s
	kubectl wait --for=condition=available deploy/shadow-web -n $(NAMESPACE) --timeout=300s
	kubectl wait --for=condition=available deploy/shadow-admin -n $(NAMESPACE) --timeout=300s
	kubectl wait --for=condition=available deploy/shadow-download -n $(NAMESPACE) --timeout=300s
	@API_IP=$$(kubectl get svc shadow-server -n $(NAMESPACE) -o jsonpath='{.spec.clusterIP}'); \
	echo "API_CLUSTER_IP=$$API_IP"; \
	kubectl run tmp --rm -it --image=curlimages/curl -n $(NAMESPACE) --restart=Never -- curl -fsS http://$$API_IP:8080/health
	@SRV_IP=$$(kubectl get svc shadow-server -n $(NAMESPACE) -o jsonpath='{.spec.clusterIP}'); \
	echo "SERVER_CLUSTER_IP=$$SRV_IP"; \
	kubectl run nctmp --rm -it --image=busybox -n $(NAMESPACE) --restart=Never -- sh -c "nc -z -v $$SRV_IP 7171 && nc -z -v $$SRV_IP 7172"

user-flow:
	@API_IP=$$(kubectl get svc shadow-server -n $(NAMESPACE) -o jsonpath='{.spec.clusterIP}'); \
	echo "API_CLUSTER_IP=$$API_IP"; \
	kubectl run curlreg --rm -it --image=curlimages/curl -n $(NAMESPACE) --restart=Never -- \
	  sh -c "curl -fsS -X POST http://$$API_IP:8080/api/v1/auth/register -H 'Content-Type: application/json' -d '{\"email\":\"testuser@example.com\",\"password\":\"Test1234!\",\"username\":\"testuser\"}'"; \
	TOKEN=$$(kubectl run curllogin --rm -it --image=curlimages/curl -n $(NAMESPACE) --restart=Never -- \
	  sh -c "curl -fsS -X POST http://$$API_IP:8080/api/v1/auth/login -H 'Content-Type: application/json' -d '{\"email\":\"testuser@example.com\",\"password\":\"Test1234!\"}' | sed -n 's/.*\\\"accessToken\\\":\\\"\([^\\\"]*\)\\\".*/\1/p'"; \
	echo "ACCESS_TOKEN_LENGTH=$${#TOKEN}"; \
	if [ -z "$$TOKEN" ]; then echo "Failed to get access token" && exit 1; fi; \
	kubectl run curlchar --rm -it --image=curlimages/curl -n $(NAMESPACE) --restart=Never -- \
	  sh -c "curl -fsS -X POST http://$$API_IP:8080/api/v1/characters -H 'Content-Type: application/json' -H 'Authorization: Bearer $$TOKEN' -d '{\"name\":\"Test Knight\",\"vocation\":\"knight\",\"sex\":\"male\",\"realm\":\"shadowveil\"}'"

ip:
	kubectl get svc -n $(NAMESPACE) -o wide

down:
	- kubectl delete -k k8s/overlays/dev || true
	- kubectl delete -k k8s/base || true
	- kind delete cluster --name $(CLUSTER) || true
