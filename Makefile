check-clippy:
	cargo clippy -- -D warnings

build-release:
	cargo build --release

test:
	cargo test