.PHONY: fmt
fmt:
	cargo +nightly fmt --all

.PHONY: clippy
clippy:
	cargo clippy --all-features --tests
