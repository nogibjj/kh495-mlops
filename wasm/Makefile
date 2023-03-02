install:
	cargo install wasm-pack

build-wasm:
	wasm-pack build --target web

serve:
	python3 -m http.server

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run