FROM golang:1.23-alpine AS builder

WORKDIR /grpc-messenger
COPY proto proto
COPY client client
RUN cd client && go mod download && go build .

FROM alpine:latest
WORKDIR /grpc-messenger
COPY --from=builder /grpc-messenger .
CMD ["./client/client"]
