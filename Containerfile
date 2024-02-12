FROM rust:alpine AS builder
RUN apk add musl-dev
WORKDIR /app
COPY . .
RUN cargo build --package relay-server --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/relay-server .
EXPOSE 80/tcp
VOLUME [ "/data" ]
CMD ["./relay-server"]