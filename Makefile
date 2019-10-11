BINARY=target/release/chip

.PHONY: build run

all: build run

build:
	cargo build --release
	strip $(BINARY)

bloat:
	cargo bloat --release

run:
	cargo run --release
