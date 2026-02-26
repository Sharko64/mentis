FROM rust:1.75-alpine AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/hello_trunk /usr/local/bin/hello_trunk
ENTRYPOINT ["/usr/local/bin/hello_trunk"]