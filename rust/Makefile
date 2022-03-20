.PHONY: clippy
clippy: check
	cargo clippy --fix

.PHONY: check
check: Cargo.toml
	cargo check

.PHONY: fmt
fmt: Cargo.toml
	cargo fmt --all -- --check
