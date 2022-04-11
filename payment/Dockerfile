FROM rust:1.59.0 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /app/target/release/payment /app/payment
RUN apt update && apt install libssl-dev -y
CMD ["/app/payment"]