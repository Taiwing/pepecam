FROM rust:1.68.2
WORKDIR /app
COPY . .
RUN cargo build --release

CMD cargo run --release
