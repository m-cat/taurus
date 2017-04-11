run: test format build clean
	cargo run

build:
	cargo build

format:
	cargo fmt # run rustfmt

test:
	cargo test

check:
	cargo check # run compiler checks without building

lint:
	cargo clippy

modules:
	cargo modules # display module structure

clean:
	rm -f -r src/*.bk
