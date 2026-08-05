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

When a reviewed submodule is adopted, a root package/composition manifest allows `zed overtake --git-submodules` to preserve `.gitmodules`, the exact gitlink, and provenance.
