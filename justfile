
# Build both Docker images inside the minikube daemon
build:
    #!/usr/bin/env bash
    set -euo pipefail
    eval "$(minikube docker-env)"
    docker build -t url-shortener:latest .
    docker build -t url-shortener-frontend:latest ./client

# Build images, apply manifests and wait for all pods to be ready
deploy:
    just build
    kubectl apply -k k8s/overlays/local
    kubectl rollout status deployment/url-shortener-backend
    kubectl rollout status deployment/url-shortener-frontend

# Tear down all resources (keeps minikube running)
delete:
    kubectl delete -k k8s/overlays/local --ignore-not-found=true
