run: format clippy test build graph clean
	cargo run

build:
	cargo build

format:
	cargo fmt

test:
	cargo test

check:
	cargo check

clippy:
	cargo clippy --profile=test

doc:
	cargo doc

# Display module structure
modules:
	cargo modules --orphans

# Run fuzz
fuzz:
	cargo fuzz run fuzz_target_1 -- -max_len=4128 -max_total_time=120

clean:
	rm **/*.bk
