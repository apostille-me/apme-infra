-- apostille-me: private application namespace inside the shared oresoftware Supabase project.
begin;

create schema if not exists apostille_me;
comment on schema apostille_me is 'apostille-me application namespace; Shared Auth remains authoritative for identity.';

revoke all on schema apostille_me from public, anon, authenticated;
alter default privileges in schema apostille_me revoke all on tables from public, anon, authenticated;
alter default privileges in schema apostille_me revoke all on sequences from public, anon, authenticated;
alter default privileges in schema apostille_me revoke all on functions from public, anon, authenticated;

commit;
