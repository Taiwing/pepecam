FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
CMD ["api"]
