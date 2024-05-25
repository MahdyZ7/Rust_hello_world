# THIS IS ONLY A TEMPLATE IT SHOULD NOT RUN

# Define the compiler
CARGO := cargo

# Define the target directory for the build artifacts
TARGET_DIR := target

# Default target executed when no arguments are given to make.
default: build

build:
	$(CARGO) build --release

clean:
	rm -rf $(TARGET_DIR)

run:
	$(CARGO) run

test:
	$(CARGO) test

clippy:
	$(CARGO) clippy

fmt:
	$(CARGO) fmt

.PHONY: build clean run test clippy fmt
