check-clippy:
	cargo +$(RUSTV) clippy --all --all-targets --all-features --tests -- -D warnings -A clippy::upper_case_acronyms

build-release:
	cargo build --release

test:
	cargo test