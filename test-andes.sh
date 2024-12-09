#!/bin/sh

export PATH=$HOME/testnumbatsdk/andestools:$PATH
cargo test --features testnumbat-wasm-debug/andes-tests

