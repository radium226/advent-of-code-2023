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
run: run-01 run-02

.PHONY: run-01
run-01: build
	cargo run "01" <"./01-part_one.txt"
	cargo run "01" <"./01-part_two.txt"

.PHONY: run-02
run-02: build
	cargo run "02" "part-one" <"./02.txt"
	cargo run "02" "part-two" <"./02.txt"