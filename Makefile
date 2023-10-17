watch:
	cargo watch -x run
run: fmt
	cargo run
test:
	cargo test
fmt:
	cargo fmt

.PHONY: watch run test fmt