# Reusable infrastructure modules

`modules/` contains provider-agnostic or provider-specific Terraform child modules that are safe to compose from more than one root.

Provider-native Git integration roots stay in their provider directories (`neon/`, `supabase/`, `cloudflare-worker/`) and call inward to these modules. Do not move a provider-owned working directory merely to satisfy the monorepo layout.

Environment orchestration roots live under `environments/`. A resource should have one authoritative root only; environment roots must not duplicate resources already owned by a provider-native root.
