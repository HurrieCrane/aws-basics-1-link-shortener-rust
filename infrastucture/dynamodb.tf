resource "aws_dynamodb_table" "shortened_link_table" {
  name           = "shortened-links"
  billing_mode   = "PROVISIONED"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = "link-hash"

  attribute {
    name = "link-hash"
    type = "S"
  }

  tags = local.tags
}