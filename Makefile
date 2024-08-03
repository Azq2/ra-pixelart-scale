RUSTUP_TOOLCHAIN := nightly
CARGO_TARGET_DIR := target

all: prepare
	export RUSTUP_TOOLCHAIN=nightly
	export CARGO_TARGET_DIR=target
	cargo build --release

prepare:
	git submodule init
	git submodule update

install:
	install -Dm0755 -t /usr/local/bin/ target/release/ra-pixelart-scale
	mkdir -p /usr/local/share/ra-pixelart-scale
	cp -r shaders/ /usr/local/share/ra-pixelart-scale/shaders/

uninstall:
	rm -f /usr/local/bin/ra-pixelart-scale
	rm -rf /usr/local/share/ra-pixelart-scale/

clean:
	cargo clean

.PHONY: all prepare install
