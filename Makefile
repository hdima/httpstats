compile:
	cargo build -v

release:
	cargo build -v --release

test:
	cargo test -v

clean:
	cargo clean -v

.PHONY: compile release test clean
