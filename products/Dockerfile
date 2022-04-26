FROM rust:1.59.0 as builder
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip /app/target/x86_64-unknown-linux-musl/release/products


# FROM debian:buster-slim as runner
FROM scratch as runner
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/products /app/products
COPY ./products.json /app/products.json
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/app/products"]