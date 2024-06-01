.PHONY: all
all:
	cargo build

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy
