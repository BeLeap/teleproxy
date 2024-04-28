FROM golang:1.18-alpine

WORKDIR /app

COPY . .
RUN go mod download

RUN go build cmd/teleproxy/main.go -o /bin/teleproxy

CMD ["/bin/teleproxy", "server", "-c /etc/teleproxy/config.yaml"]