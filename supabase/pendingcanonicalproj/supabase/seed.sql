-- Preview/local seed for apostille-me (canonical). Runs only on local stacks and Supabase preview branches.
-- Keep it idempotent, synthetic, and inside the apostille_me schema. Never add real user data or credentials.
create schema if not exists apostille_me;
