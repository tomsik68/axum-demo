FROM lukemathwalker/cargo-chef:latest-rust-1.56.0 AS chef

RUN wget https://github.com/mozilla/sccache/releases/download/v0.2.15/sccache-v0.2.15-x86_64-unknown-linux-musl.tar.gz \
    && tar xzf sccache-v0.2.15-x86_64-unknown-linux-musl.tar.gz \
    && mv sccache-v0.2.15-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache \
    && chmod +x /usr/local/bin/sccache
ENV RUSTC_WRAPPER=/usr/local/bin/sccache

WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin axum-demo

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc AS runtime
WORKDIR app
COPY --from=builder /app/target/release/axum-demo /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]
EXPOSE 8080
