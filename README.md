# apme-infra

Docker Compose, Kubernetes, Kustomize, Argo CD, Terraform, observability, and runbooks for Apostille Me.

**Product:** Apostille Me — Case operations for visa and apostille consulting.

Track sanitized client references, document workflows, destination jurisdictions, appointments, deadlines, and case events for a visa and apostille consulting firm.

## Safety and production boundary

This software is an operational starter and does not provide legal advice. Keep identity documents and sensitive case files out of logs and this bootstrap data model; production use requires encryption, access controls, retention rules, auditability, and jurisdiction-specific professional review.

This repository is an executable bootstrap, not a production deployment. Before live
use, add authentication, tenant authorization, rate limits, durable migrations,
observability, backups, incident response, dependency review, and secret management.
            ## Services

            - `apme-api`
- `apme-web-mash`
- `apme-web-leptos`
- `apme-web-dioxus`
- `apme-sync`

            The checked-in images use version tags as placeholders. Production GitOps must pin
            immutable digests produced by verified CI, use an external secrets provider, and
            configure managed PostgreSQL/Supabase, backups, TLS, network policy, autoscaling,
            dashboards, and alerts.

            ```bash
            ./scripts/validate.sh
            docker compose up
            ```

## Cloudflare Worker edge gateway

The `cloudflare-worker/` package provides a Wrangler-managed edge gateway with health checks, signed webhook intake, validation, queue fan-out, security headers, unit tests, and a dry-run deployment command. The Worker is intentionally isolated from cluster infrastructure so it can be reviewed and deployed independently.
