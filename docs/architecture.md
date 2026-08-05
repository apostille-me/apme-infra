# Architecture

`apme-infra` owns Kubernetes, Argo CD, local composition, observability, deployment catalogs, and bounded Cloudflare Worker edge code for Apostille Me.

## Canonical boundaries

- application packages resolve through the short `apostille-me/apme-*` Zed coordinates
- infrastructure composes deployments and must not become a runtime package dependency
- edge code is explicitly allowlisted and must never become a generic proxy
- secrets stay in deployment secret stores and are never committed to manifests or fixtures

The long `apostille-me-infra` repository is a historical bootstrap alias. Its small Worker scaffold is intentionally not copied because `apme-infra` contains the maintained Worker, security helpers, deployment workflows, Kubernetes resources, and tests.

## Zed and Git submodules

Use Zed for application dependencies. Retain infrastructure as a Git submodule only for an explicit editable-deployment, inventory, or legacy role. Do not resolve the same repository through both Zed and a gitlink in one composition.

`zed overtake --git-submodules` adopts only initialized submodules that declare their own `.zpkg.toml`; it writes them into the root manifest and lockfile while retaining `.gitmodules` as a reversible transport mirror and recording exact gitlinks and provenance. Infrastructure submodules without a package manifest remain under Git authority.
