run: format test build clean
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

doc:
	cargo doc

modules:
	cargo modules # display module structure

clean:
	rm -f -r src/*.bk
