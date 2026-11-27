#!/usr/bin/env bash
# Builds the engine for the browser: web/src/engine-wasm/ (generated, not in git),
# and the engine with OCR, web/src/ocr-wasm/, which the page fetches only when a
# visitor asks for a scanned PDF to be read.
#
#   bash scripts/build-wasm.sh
#
# Needs the wasm32-unknown-unknown target and wasm-bindgen-cli at the version
# Cargo.lock pins for wasm-bindgen (see README, "Run it locally").
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release --lib --target wasm32-unknown-unknown --no-default-features --features wasm
wasm-bindgen --target web --out-dir web/src/engine-wasm --out-name engine \
  target/wasm32-unknown-unknown/release/deid_server.wasm
cargo build --release --lib --target wasm32-unknown-unknown --no-default-features --features wasm,ocr
wasm-bindgen --target web --out-dir web/src/ocr-wasm --out-name ocr \
  target/wasm32-unknown-unknown/release/deid_server.wasm
