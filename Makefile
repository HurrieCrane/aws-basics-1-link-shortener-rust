.PHONY: build

build:
	cargo lambda build --release --target aarch64-unknown-linux-gnu

start:
	cargo lambda start

invoke_generator_200:
	cargo lambda invoke generator --data-file examples/200_generator.json

invoke_resolver_200:
	cargo lambda invoke resolver --data-file examples/200_resolver.json