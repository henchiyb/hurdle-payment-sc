#!/bin/bash
set -e

RSUTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p out
cp target/wasm32-unknown-unknown/release/*.wasm out/hurdle-payment.wasm