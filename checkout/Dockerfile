
FROM golang:1.18 as builder

WORKDIR /go/src/
COPY . .
RUN GOOS=linux go build -ldflags="-s -w"

FROM alpine:3.15.3
WORKDIR /app
COPY --from=builder /go/src/checkout /app/checkout
CMD ["./checkout"]
