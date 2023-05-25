#!/usr/bin/env bash

cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/vulgar_fractions.wasm public/vulgar_fractions.wasm
