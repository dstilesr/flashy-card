FROM rust:1-alpine AS builder

WORKDIR /build

COPY Cargo.toml .
COPY Cargo.lock .

COPY templates ./templates
COPY src ./src

RUN cargo build --release

FROM alpine:latest AS runner

WORKDIR /app

COPY static ./static
COPY migrations ./migrations
COPY --from=builder /build/target/release/flashy-card .

CMD ["./flashy-card"]
