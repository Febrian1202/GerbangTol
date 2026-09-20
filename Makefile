.PHONY: dev check

dev:
	cargo watch -x run

check:
	cargo check
