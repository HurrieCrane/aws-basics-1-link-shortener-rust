.PHONY: build

build:
	cargo build

start:
	cargo lambda start

invoke_generator_200:
	cargo lambda invoke link-generator --data-file examples/200_generator.json

invoke_resolver_200:
	cargo lambda invoke link-resolver --data-file examples/200_resolver.json