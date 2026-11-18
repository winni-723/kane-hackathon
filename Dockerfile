# One image, one service: the Axum server also serves the built front end.

# --- front end ---------------------------------------------------------------
FROM node:22-bookworm-slim AS web
WORKDIR /app/web
COPY web/package.json web/package-lock.json ./
RUN npm ci
COPY web/ ./
RUN npm run build

# --- server ------------------------------------------------------------------
FROM rust:1-slim-bookworm AS server
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# The PDF library, patched; Cargo.toml points pdf-extract here.
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
