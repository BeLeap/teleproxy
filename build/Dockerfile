FROM golang:1.22.1-bookworm AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN CGO_ENABLED=0 go build ./cmd/teleproxy

FROM gcr.io/distroless/static-debian12

COPY examples/config/config.yaml /etc/teleproxy/config.yaml
COPY --from=builder /app/teleproxy /bin/teleproxy

ENTRYPOINT ["/bin/teleproxy", "server", "-c", "/etc/teleproxy/config.yaml"]
