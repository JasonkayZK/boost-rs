.PHONY: fmt clippy test fix clean build all ci

all: clean fmt clippy

ci: fmt clippy test

fmt:
	cargo fmt --all --

clippy:
	cargo clippy --all -- -D warnings

test:
	cargo test --all --
fix:
	cargo +nightly clippy --all --fix -Z unstable-options --allow-staged

clean:
	rm -rf ./target && rm -rf ./*/target

build:
	cargo build --all
