resource "aws_apigatewayv2_api" "shortener" {
  name          = "link-shortener"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "live" {
  api_id      = aws_apigatewayv2_api.shortener.id
  name        = "$default"
  auto_deploy = true
}

# ===== GENERATOR ===== #

resource "aws_apigatewayv2_integration" "generate_integration" {
  api_id    = aws_apigatewayv2_api.shortener.id
  integration_type = "AWS_PROXY"

  integration_method   = "POST"
  integration_uri      = aws_lambda_function.link_generator.invoke_arn
  passthrough_behavior = "WHEN_NO_MATCH"
}

resource "aws_apigatewayv2_route" "generator_route" {
  api_id             = aws_apigatewayv2_api.shortener.id
  route_key          = "GET /"
  target             = "integrations/${aws_apigatewayv2_integration.generate_integration.id}"
}

# ===== RESOLVER ===== #

resource "aws_apigatewayv2_integration" "resolver_integration" {
  api_id    = aws_apigatewayv2_api.shortener.id
  integration_type = "AWS_PROXY"

  integration_method   = "POST"
  integration_uri      = aws_lambda_function.link_resolver.invoke_arn
  passthrough_behavior = "WHEN_NO_MATCH"
}

resource "aws_apigatewayv2_route" "resolver_route" {
  api_id             = aws_apigatewayv2_api.shortener.id
  route_key          = "GET /link/{hash}"
  target             = "integrations/${aws_apigatewayv2_integration.resolver_integration.id}"
}