format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet
run:
	cargo run 

release:
	cargo lambda build --release
	
release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"tz_code": "AEST"}' \
  		--output-format json \
  		worldclock

all: format lint test run