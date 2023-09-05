#!/bin/bash

all:
	SLINT_STYLE=fluent cargo build --release

build:
	SLINT_STYLE=fluent cargo build --release

build-timings:
	SLINT_STYLE=fluent cargo build --release --timings
	cp -rf ./target/cargo-timings/cargo-timing.html ./profile

build-debug:
	SLINT_STYLE=fluent cargo build

run:
	SLINT_STYLE=fluent RUST_LOG=error,warn,info,debug,reqwest=on cargo run

run-local:
	RUST_LOG=error,warn,info,debug,reqwest=on ./target/debug/chatbox

run-local-release:
	RUST_LOG=error,warn,info,debug,reqwest=on ./target/release/chatbox

clippy:
	cargo clippy

clean-incremental:
	rm -rf ./target/debug/incremental/*

clean:
	cargo clean

install:
	cp -rf ./target/release/chatbox ~/bin/

slint-view:
	# slint-viewer --style native --auto-reload -I chatbox/ui ./chatbox/ui/appwindow.slint
	# slint-viewer --style material --auto-reload -I chatbox/ui ./chatbox/ui/appwindow.slint
	# slint-viewer --style cupertino --auto-reload -I chatbox/ui ./chatbox/ui/appwindow.slint
	slint-viewer --style fluent --auto-reload -I chatbox/ui ./chatbox/ui/appwindow.slint
