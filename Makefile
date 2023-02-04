.PHONY: build

build:
	cargo build

start:
	cargo lambda start

invoke_example:
	cargo lambda invoke lambda-1 --data-file examples/200.json