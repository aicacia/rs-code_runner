.PHONY: default all release

default: release
all: default

release:
	cargo test
	cargo build --release
	cp target/release/runner bin
	chmod +x bin/runner