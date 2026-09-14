terraform {
  required_version = ">= 1.12.0, < 2.0.0"

  required_providers {
    neon = {
      source  = "kislerdm/neon"
      version = "= 0.15.0"
    }
  }
}

provider "neon" {} # NEON_API_KEY comes from ores-sops at reviewed plan/apply time; never commit it.

module "project" {
  source = "../../../modules/neon-project"

  neon_org_id         = var.neon_org_id
  project_name        = "apostille-me-prod"
  region_id           = "aws-us-east-2"
  canonical_role_name = "apostille_me_app"
  auth_role_name      = "apostille_me_auth"
}
