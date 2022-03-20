.PHONY : check
check: rust-check

.PHONY : rust-check
rust-check:
	$(MAKE) -C rust/ clippy