SHELL := /bin/bash

# Configuration
CLUSTER ?= shadow-local
NAMESPACE ?= shadow-ot
DOCKER_COMPOSE := docker compose -f docker/docker-compose.yml

.PHONY: up down status logs health ps \
        k8s-up k8s-down kind metallb build-images load-images deploy k8s-health user-flow ip ws-check monitoring \
        db-migrate db-reset sprites help

# =============================================================================
# DEFAULT: Docker Compose (Simple Local Development)
# =============================================================================

## Start all services (PostgreSQL, Redis, Server, Web, Admin, Monitoring)
up:
	@echo "Starting Shadow OT with Docker Compose..."
	$(DOCKER_COMPOSE) up -d
	@echo ""
	@echo "Waiting for services to be healthy..."
	@sleep 5
	@$(MAKE) health
	@echo ""
	@$(MAKE) status

## Stop all services
down:
	@echo "Stopping Shadow OT..."
	$(DOCKER_COMPOSE) down

## Show service status
status:
	@echo "=== Shadow OT Service Status ==="
	@$(DOCKER_COMPOSE) ps
	@echo ""
	@echo "=== Service URLs ==="
	@echo "  Web Frontend:   http://localhost:3000"
	@echo "  Admin Panel:    http://localhost:3001"
	@echo "  REST API:       http://localhost:8080"
	@echo "  API Health:     http://localhost:8080/health"
	@echo "  WebSocket:      ws://localhost:8081"
	@echo "  Login Server:   localhost:7171"
	@echo "  Game Server:    localhost:7172"
	@echo "  Grafana:        http://localhost:3002 (admin/admin)"
	@echo "  Prometheus:     http://localhost:9091"
	@echo ""

## Show service logs (follow mode)
logs:
	$(DOCKER_COMPOSE) logs -f

## Show logs for specific service (usage: make logs-server, logs-web, etc.)
logs-%:
	$(DOCKER_COMPOSE) logs -f $*

## List running containers
ps:
	$(DOCKER_COMPOSE) ps

## Health check all services
health:
	@echo "Checking service health..."
	@$(DOCKER_COMPOSE) ps --format "table {{.Name}}\t{{.Status}}" | grep -E "(healthy|running)" || true
	@echo ""
	@echo "Checking API health endpoint..."
	@curl -sf http://localhost:8080/health 2>/dev/null && echo "API: OK" || echo "API: Not ready (server may still be starting)"
	@echo ""
	@echo "Checking database..."
	@docker exec shadow-postgres pg_isready -U shadow -d shadow_ot 2>/dev/null && echo "PostgreSQL: OK" || echo "PostgreSQL: Not ready"
	@echo ""
	@echo "Checking Redis..."
	@docker exec shadow-redis redis-cli ping 2>/dev/null && echo "Redis: OK" || echo "Redis: Not ready"

# =============================================================================
# DATABASE OPERATIONS
# =============================================================================

## Apply database migrations
db-migrate:
	@echo "Applying database migrations..."
	@for f in crates/shadow-db/migrations/*.sql; do \
		echo "Applying $$f..."; \
		docker exec -i shadow-postgres psql -U shadow -d shadow_ot < "$$f" 2>&1 | grep -v "already exists" | grep -v "duplicate key" || true; \
	done
	@echo "Migrations complete."

## Reset database (WARNING: destroys all data)
db-reset:
	@echo "WARNING: This will destroy all data in the database!"
	@read -p "Are you sure? [y/N] " confirm && [ "$$confirm" = "y" ] || exit 1
	@docker exec shadow-postgres psql -U shadow -d shadow_ot -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
	@$(MAKE) db-migrate

# =============================================================================
# SPRITE ASSETS
# =============================================================================

## Download sprite assets (required for game client)
sprites:
	@echo "Downloading sprite assets..."
	@mkdir -p client/data/sprites
	@if [ ! -f client/data/sprites/Tibia.spr ]; then \
		echo "Downloading from ots.me..."; \
		curl -L -o /tmp/sprites_1287.zip "https://downloads.ots.me/data/tibia-clients/dat_and_spr/1287.zip" && \
		unzip -o /tmp/sprites_1287.zip -d /tmp/sprites_tmp && \
		mv /tmp/sprites_tmp/1287/Tibia.dat client/data/sprites/ && \
		mv /tmp/sprites_tmp/1287/Tibia.spr client/data/sprites/ && \
		rm -rf /tmp/sprites_1287.zip /tmp/sprites_tmp && \
		echo "Sprites downloaded successfully!"; \
	else \
		echo "Sprites already exist in client/data/sprites/"; \
	fi

# =============================================================================
# KUBERNETES (Production-like Local Environment)
# =============================================================================

## Start with Kubernetes (Kind + MetalLB)
k8s-up: kind metallb build-images load-images deploy k8s-health user-flow ip

## Stop Kubernetes cluster
k8s-down:
	-kubectl delete -k k8s/overlays/dev || true
	-kubectl delete -k k8s/base || true
	-kind delete cluster --name $(CLUSTER) || true

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

k8s-health:
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

ws-check:
	@SRV_IP=$$(kubectl get svc shadow-server -n $(NAMESPACE) -o jsonpath='{.status.loadBalancer.ingress[0].ip}'); \
	echo "WS_EXTERNAL_IP=$$SRV_IP"; \
	R1=$$(printf "GET /ws HTTP/1.1\r\nHost: ws.shadow-ot.com\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n" | nc -w 5 $$SRV_IP 8081 | head -n 1 | grep -c "101" || true); \
	R2=$$(printf "GET / HTTP/1.1\r\nHost: ws.shadow-ot.com\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n" | nc -w 5 $$SRV_IP 8081 | head -n 1 | grep -c "101" || true); \
	if [ "$$R1" = "1" ] || [ "$$R2" = "1" ]; then echo "WebSocket handshake OK"; else echo "WebSocket handshake failed" && exit 1; fi

monitoring:
	kubectl apply -f k8s/base/monitoring.yaml

# =============================================================================
# HELP
# =============================================================================

## Show this help
help:
	@echo "Shadow OT - Makefile Commands"
	@echo ""
	@echo "=== Quick Start (Docker Compose) ==="
	@echo "  make up          Start all services"
	@echo "  make down        Stop all services"
	@echo "  make status      Show service status and URLs"
	@echo "  make logs        Follow all service logs"
	@echo "  make health      Check service health"
	@echo ""
	@echo "=== Database ==="
	@echo "  make db-migrate  Apply database migrations"
	@echo "  make db-reset    Reset database (WARNING: destroys data)"
	@echo ""
	@echo "=== Assets ==="
	@echo "  make sprites     Download sprite files for game client"
	@echo ""
	@echo "=== Kubernetes (Advanced) ==="
	@echo "  make k8s-up      Start with Kind + MetalLB"
	@echo "  make k8s-down    Stop Kubernetes cluster"
	@echo ""
	@echo "=== First Time Setup ==="
	@echo "  1. make sprites  (download game assets)"
	@echo "  2. make up       (start all services)"
	@echo "  3. make status   (verify everything is running)"
	@echo ""
