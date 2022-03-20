.PHONY: all
all: rust-build

.PHONY: check
check: rust-check

.PHONY: lint
lint: rust-lint

.PHONY: clean
clean: rust-clean

.PHONY: rust-check
rust-check:
	$(MAKE) -C rust/ clippy

.PHONY: rust-lint
rust-lint:
	$(MAKE) -C rust/ fmt

.PHONY: rust-build
rust-build:
	$(MAKE) -C rust/ build

.PHONY: rust-clean
rust-clean:
	$(MAKE) -C rust/ clean