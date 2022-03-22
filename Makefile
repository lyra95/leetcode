.PHONY: all
all: rust-build

.PHONY: check
check: rust-check

.PHONY: lint
lint: rust-lint python-lint

.PHONY: test
test: rust-test python-test

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

.PHONY: rust-test
rust-test:
	$(MAKE) -C rust/ test

.PHONY: python-lint
python-lint:
	$(MAKE) -C python3/ lint

.PHONY: python-test
python-test:
	$(MAKE) -C python3/ test
