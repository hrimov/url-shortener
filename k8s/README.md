# Kubernetes setup

Local stack runs on **minikube** with **kustomize** overlays.

## Overlays

| Overlay | Path                 | Purpose              |
|---------|----------------------|----------------------|
| `local` | `k8s/overlays/local` | minikube development |

## Secrets

`k8s/overlays/local/secret.yaml` contains local-only credentials
(`postgres/postgres`) and is safe to commit. For other environments, 
create a new overlay with real credentials and add it to `.gitignore`.

