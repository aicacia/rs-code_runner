.PHONY: default all release debug

default: release
all: default

release:
	cargo test
	cargo build --release
	cp target/release/runner bin
	chmod +x bin/runner

debug:
	cargo build
	cp target/debug/runner /home/nathan/elixir_code/ex-runner/priv/containers/node/latest
	cp target/debug/runner /home/nathan/elixir_code/ex-runner/priv/containers/rust/latest
	chmod +x /home/nathan/elixir_code/ex-runner/priv/containers/node/latest
	chmod +x /home/nathan/elixir_code/ex-runner/priv/containers/rust/latest