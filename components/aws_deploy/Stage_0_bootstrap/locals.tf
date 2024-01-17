variable "environment" {
  type = string
  description = "Environment to select for deployment, use \"testing\" or \"prod\" via \"export TF_VAR_environment=prod\" Defaults to \"testing\""
  default = "testing"
}

locals {
  environment_filename = format("%s/../environment/%s.json", "${path.module}", var.environment)
  environment = jsondecode(file(local.environment_filename))  
}