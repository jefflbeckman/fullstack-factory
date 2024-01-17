terraform {
  required_version = "~> 1.3"

  backend "s3" {
    bucket         = "jbtf-state"
    key            = "tf-infra/terraform.tfstate"
    region         = "us-west-2"
    dynamodb_table = "jbtfStateTable"
    encrypt        = true
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

module "tf-state" {
  source      = "../modules/tf-state"
  bucket_name = local.environment.state_bucket
  table_name  = local.environment.state_table
}