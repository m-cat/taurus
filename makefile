run: test build
	cargo run

build: format
	cargo build

format:
	cargo fmt    # run rustfmt

test:
	cargo test

clippy:
	cargo clippy # run clippy

clean:
	rm src/*.bk
