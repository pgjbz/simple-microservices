FROM rust:1.59.0 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/products /usr/local/bin/products
COPY ./products.json /app/products.json
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["products"]