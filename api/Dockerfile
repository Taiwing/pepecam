FROM lukemathwalker/cargo-chef:0.1.61-rust-1.70-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin api

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
RUN apt update && apt install -y libssl-dev ca-certificates
COPY --from=builder /app/target/release/api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/api"]
