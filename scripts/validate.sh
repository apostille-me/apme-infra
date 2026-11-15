#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
cargo run --manifest-path tools/infra-contract-check/Cargo.toml -- --root .
if command -v kubectl >/dev/null; then
  kubectl kustomize k8s/base >/dev/null
  kubectl kustomize k8s/overlays/dev >/dev/null
fi
if command -v terraform >/dev/null; then
  terraform -chdir=terraform fmt -check
  terraform -chdir=terraform init -backend=false -input=false >/dev/null
  terraform -chdir=terraform validate
fi
echo "infrastructure bootstrap validation passed"
