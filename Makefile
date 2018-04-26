.PHONY: default all release

default: release
all: default

release:
	cargo test
	cargo build --release
	cp target/release/code_runner bin
	chmod +x bin/code_runner
