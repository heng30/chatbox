#!/bin/bash

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")

# RUST_BACKTRACE=full RUST_LOG=error,warn,info,debug  $DIR/chatbox
RUST_BACKTRACE=full RUST_LOG=error,warn,info,debug  $DIR/chatbox 2&> $DIR/chatbox.log &
