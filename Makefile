all: target/release/cf-linux

RUST_SRC := $(shell find src -type f -name \*.rs)

target/release/cf-linux: ${RUST_SRC}
	RUSTFLAGS='-C link-args=-s' cargo build --release