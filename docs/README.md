# Client

## Usage

```sh
Usage:
  teleproxy client [flags]
  teleproxy client [command]

Available Commands:
  admin       

Flags:
  -a, --addr string     server addr (default "127.0.0.1:4001")
      --apikey string   api key
  -h, --help            help for client
  -k, --key string      Header Key to Spy (default "User-No")
  -t, --target string   Target
  -v, --value string    Header Value to Spy

Use "teleproxy client [command] --help" for more information about a command.
```

## Installation

### macOS

#### Using homebrew

```sh
brew install beleap/tap/teleproxy
```

#### Download from release

1. Download appropriate tar
2. Extarct
3. Move `bin/teleproxy` to somewhere in your `$PATH`
    1. If you get an error "is damaged and can’t be opened." when opening, run following command.
        `xattr -cr <path-to-teleproxy>`

# Server

## Usage

```sh
Usage:
  teleproxy server [flags]

Flags:
  -c, --config string   path for config file
  -h, --help            help for server
  -l, --port int        listening port (default 4001)
  -p, --proxyPort int   proxing port (default 4000)
  -t, --target string   target (default "http://localhost:8080")
```

Default config file location for Docker image is `/etc/teleproxy/config.yaml`.

`API_KEY` environment variable is required.
Use any random value. This value is used to authenticate client.

> [!NOTE]
> Use `openssl rand -hex 8` to generate appropriate random value.

For avaliable values for `config.yaml` see [`examples/config/config.yaml`](../examples/config/config.yaml).
