install:
	cargo build

rust-version:
	rustc --version
	cargo --version

test:
	cargo test

format:
	cargo fmt

lint:
	cargo clippy

all: install lint format test

setup_package:
	cargo install --path .

