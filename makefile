run: format build clean
	cargo run

build:
	cargo build

format:
	cargo fmt # run rustfmt

test:
	cargo test

lint:
	cargo clippy

clean:
	rm src/*.bk
