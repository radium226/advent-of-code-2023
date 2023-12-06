.PHONY: clean
clean:
	cargo clean

.PHONY: test
test:
	cargo test

.PHONY: build
bulid: test
	cargo build

.PHONY: run
run: build
	cargo run <"./01-part_one.txt"
	cargo run <"./01-part_two.txt"