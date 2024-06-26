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
  -h, --help            help for client
  -i, --insecure        Use insecure connection
      --key string      Header Key to Spy (default "User-No")
  -t, --target string   Target
      --value string    Header Value to Spy

Global Flags:
      --apikey string   api key for auth
  -v, --verbose         verbose

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
2. Extract
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

Global Flags:
      --apikey string   api key for auth
  -v, --verbose         verbose
```

Default config file location for Docker image is `/etc/teleproxy/config.yaml`.

`apikey` is required.
Use any random value. This value is used to authenticate client.

> [!NOTE]
> Use `openssl rand -hex 8` to generate appropriate random value.

For avaliable values for `config.yaml` see [`examples/config/config.yaml`](../examples/config/config.yaml).
