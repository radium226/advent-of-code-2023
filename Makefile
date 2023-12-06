.PHONY: test
test:
	cargo test

.PHONY: build
bulid: test
	cargo build

.PHONY: run
run: build
	cargo run <"./01.txt"