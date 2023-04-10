#!/bin/bash

all:
	cargo build --release

build:
	cargo build --release

build-debug:
	cargo build

run:
	RUST_LOG=error,warn,info,debug,reqwest=on proxychains cargo run

clippy:
	cargo clippy

clean:
	cargo clean


slint-view:
	slint-viewer --auto-reload -I chatbox/ui ./chatbox/ui/appwindow.slint
