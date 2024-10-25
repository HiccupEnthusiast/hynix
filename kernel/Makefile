RUST_TARGET ?= x86_64-unknown-none

RUST_PROFILE ?= dev

override RUST_PROFILE_DIR = $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
	override RUST_PROFILE_DIR := debug
endif

.PHONY: all
all: 
	RUSTFLAGS="-C relocation-model=static" cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE)
	cp target/$(RUST_TARGET)/$(RUST_PROFILE_DIR)/hynix-kernel kernel

.PHONY: clean 
clean: 
	rm -rf target 
	rm kernel
