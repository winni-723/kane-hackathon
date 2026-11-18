# One image, one service. The engine is compiled to WebAssembly and runs in
# the visitor's browser; the server only hands out the page's files.

# --- engine for the browser ----------------------------------------------------
FROM rust:1-slim-bookworm AS engine
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
COPY Cargo.toml Cargo.lock ./
# wasm-bindgen-cli must be the version Cargo.lock pins for wasm-bindgen.
RUN version=$(grep -A1 '^name = "wasm-bindgen"$' Cargo.lock | sed -n 's/^version = "\(.*\)"/\1/p') \
 && cargo install wasm-bindgen-cli --version "$version" --locked
# The PDF library, patched; Cargo.toml points pdf-extract here.
COPY vendor ./vendor
COPY src ./src
RUN cargo build --release --lib --target wasm32-unknown-unknown --no-default-features --features wasm \
 && wasm-bindgen --target web --out-dir /engine-wasm --out-name engine \
      target/wasm32-unknown-unknown/release/deid_server.wasm

# --- front end -----------------------------------------------------------------
FROM node:22-bookworm-slim AS web
WORKDIR /app/web
COPY web/package.json web/package-lock.json ./
RUN npm ci
COPY web/ ./
COPY --from=engine /engine-wasm ./src/engine-wasm
RUN npm run build

# --- server: serves web/dist and nothing else -------------------------------
FROM rust:1-slim-bookworm AS server
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY vendor ./vendor
COPY src ./src
RUN cargo build --release --bin deid-server

# --- runtime -----------------------------------------------------------------
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=server /app/target/release/deid-server /app/deid-server
COPY --from=web /app/web/dist /app/web/dist
# The host sets PORT; 8080 is only the default for running the image by hand.
ENV PORT=8080 \
    STATIC_DIR=/app/web/dist
EXPOSE 8080
USER 65534:65534
CMD ["/app/deid-server"]
