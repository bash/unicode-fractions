#!/usr/bin/env bash

cargo build --release --target wasm32-unknown-unknown -p vulgar-fractions-wasm
cp target/wasm32-unknown-unknown/release/vulgar_fractions_wasm.wasm web/public/vulgar_fractions.wasm
