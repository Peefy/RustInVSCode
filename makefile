default: run

run:
	cargo run --release

check:
	cargo check --release

run-bin:
	./target/release/hello_cargo

build:
	cargo build --release

format:
	cargo fmt

fmt: format
