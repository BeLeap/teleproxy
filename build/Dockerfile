FROM golang:1.22.1-bookworm AS builder

WORKDIR /app

COPY . .
RUN go mod download

RUN go build ./cmd/teleproxy

FROM gcr.io/distroless/static-debian12:latest

COPY --from=builder /app/teleproxy /bin/teleproxy

CMD ["/bin/teleproxy", "server", "-c /etc/teleproxy/config.yaml"]
