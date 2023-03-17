#!/usr/bin/env bash
rustup default nightly
rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
