FROM rust:1.61 as builder
RUN cargo new api
COPY Cargo.toml Cargo.lock /api/
WORKDIR /api/
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release
COPY src/* /api/src/
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install --path .

FROM debian:buster-slim
RUN apt update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
COPY Rocket.toml /
CMD ["api"]
