data "aws_caller_identity" "current" {}

locals {
  tags = {
    Project = "link-shortener"
  }
}