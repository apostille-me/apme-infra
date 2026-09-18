variable "neon_org_id" {
  description = "Provider-read Neon organization ID; never guess it or commit a credential."
  type        = string
  nullable    = false

  validation {
    condition     = can(regex("^org-[a-z0-9-]+$", var.neon_org_id))
    error_message = "neon_org_id must be a provider-read Neon organization ID beginning with org-."
  }
}

variable "project_name" {
  type = string
}

variable "region_id" {
  type = string
}

variable "production_branch_name" {
  type    = string
  default = "production"
}

variable "canonical_database_name" {
  type    = string
  default = "canonical"
}

variable "canonical_role_name" {
  type = string
}

variable "auth_database_name" {
  type    = string
  default = "auth"
}

variable "auth_role_name" {
  type = string
}
