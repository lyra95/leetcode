.PHONY: check
check: rust-check

.PHONY: lint
lint: rust-lint

.PHONY: rust-check
rust-check:
	"$(MAKE)" -C rust/ clippy

.PHONY: rust-lint
rust-lint:
	"$(MAKE)" -C rust/ fmt