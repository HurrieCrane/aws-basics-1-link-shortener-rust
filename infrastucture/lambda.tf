# ===== LINK GENERATOR ===== #

resource "aws_iam_policy" "link_generator_policy" {
  name        = "link-generator-dynamo-policy"
  description = "A policy that allows writing to the shortened-links table"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "dynamodb:PutItem",
        ]
        Effect   = "Allow"
        Resource = aws_dynamodb_table.shortened_link_table.arn
      },
    ]
  })
}

resource "aws_iam_role" "link_generator_role" {
  name = "link_generator_assume_role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": "DefaultAssumeRole"
    }
  ]
}
EOF

  tags = local.tags
}

resource "aws_iam_policy_attachment" "generator_policy_attachment" {
  name       = "generator_policy_attachment"
  policy_arn = aws_iam_policy.link_generator_policy.arn
  roles = [
    aws_iam_role.link_generator_role.name
  ]
}

data "archive_file" "generator_zip" {
  type        = "zip"
  output_path = "generator.zip"

  source_dir = "../target/lambda/generator"
}

resource "aws_lambda_function" "link_generator" {
  function_name = "link-generator"
  description   = "A Lambda function which will generate shortened links"

  handler  = "bootstrap"
  runtime  = "provided.al2"
  filename = data.archive_file.generator_zip.output_path
  architectures = [
    "arm64"
  ]

  role = aws_iam_role.link_generator_role.arn

  source_code_hash = data.archive_file.generator_zip.output_base64sha256

  environment {
    variables = {
      LINK_DOMAIN = aws_apigatewayv2_stage.live.invoke_url
    }
  }

  tags = local.tags
}

resource "aws_lambda_permission" "generator_allow_invoke" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.link_generator.arn
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.shortener.execution_arn}/*/*/*"
}

# ===== LINK RESOLVER ===== #

resource "aws_iam_policy" "link_resolver_policy" {
  name        = "link-resolver-dynamo-policy"
  description = "A policy that allows reading from the shortened-links table"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "dynamodb:GetItem",
        ]
        Effect   = "Allow"
        Resource = aws_dynamodb_table.shortened_link_table.arn
      },
    ]
  })
}

resource "aws_iam_role" "link_resolver_role" {
  name = "link_resolver_assume_role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": "DefaultAssumeRole"
    }
  ]
}
EOF

  tags = local.tags
}

resource "aws_iam_policy_attachment" "resolver_policy_attachment" {
  name       = "resolver_policy_attachment"
  policy_arn = aws_iam_policy.link_resolver_policy.arn
  roles = [
    aws_iam_role.link_resolver_role.name
  ]
}

data "archive_file" "resolver_zip" {
  type        = "zip"
  output_path = "resolver.zip"

  source_dir = "../target/lambda/resolver"
}

resource "aws_lambda_function" "link_resolver" {
  function_name = "link-resolver"
  description   = "A function which will resolve shortened links"

  handler  = "bootstrap"
  runtime  = "provided.al2"
  filename = data.archive_file.resolver_zip.output_path
  architectures = [
    "arm64"
  ]

  role = aws_iam_role.link_resolver_role.arn

  source_code_hash = data.archive_file.resolver_zip.output_base64sha256

  environment {
    variables = {
      LINK_DOMAIN = aws_apigatewayv2_stage.live.invoke_url
    }
  }

  tags = local.tags
}

resource "aws_lambda_permission" "resolver_allow_invoke" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.link_resolver.arn
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.shortener.execution_arn}/*/*/*"
}