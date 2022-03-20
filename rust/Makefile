.PHONY: clippy
clippy: check
	cargo clippy

.PHONY: check
check: Cargo.toml
	cargo check

.PHONY: fmt
fmt: Cargo.toml
	cargo fmt --all -- --check

.PHONY: build
build:
	cargo build

.PHONY: clean
clean:
	cargo clean
