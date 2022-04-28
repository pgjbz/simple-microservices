
FROM docker.io/golang:1.18-alpine as builder

WORKDIR /go/src/
COPY . .
RUN GOOS=linux go build -ldflags="-s -w"
CMD ["./checkout"]
