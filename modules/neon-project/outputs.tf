output "project_id" {
  value = neon_project.this.id
}

output "default_branch_id" {
  value = neon_project.this.default_branch_id
}

output "auth_database_id" {
  value = neon_database.auth.id
}
