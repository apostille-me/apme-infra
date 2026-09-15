terraform {
  required_version = ">= 1.12.0, < 2.0.0"

  required_providers {
    neon = {
      source  = "kislerdm/neon"
      version = "= 0.15.0"
    }
  }
}

resource "neon_project" "this" {
  name                     = var.project_name
  org_id                   = var.neon_org_id
  region_id                = var.region_id
  default_branch_protected = true

  branch {
    name          = var.production_branch_name
    database_name = var.canonical_database_name
    role_name     = var.canonical_role_name
  }

  lifecycle {
    prevent_destroy = true
  }
}

resource "neon_role" "auth" {
  project_id = neon_project.this.id
  branch_id  = neon_project.this.default_branch_id
  name       = var.auth_role_name
}

resource "neon_database" "auth" {
  project_id = neon_project.this.id
  branch_id  = neon_project.this.default_branch_id
  name       = var.auth_database_name
  owner_name = neon_role.auth.name
}
