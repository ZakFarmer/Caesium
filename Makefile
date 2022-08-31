check-clippy:
	cargo clippy --all-targets --all-features -- -D warnings

build-release:
	cargo build --release

test:
	cargo test