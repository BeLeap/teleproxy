FROM golang:1.22.1-bookworm

WORKDIR /app

COPY . .

RUN mkdir -p /etc/teleproxy
RUN cp examples/config/config.yaml /etc/teleproxy/config.yaml

RUN go mod download

RUN go build ./cmd/teleproxy

CMD ["/app/teleproxy", "server", "-c", "/etc/teleproxy/config.yaml"]
