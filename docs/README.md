# Installation

## macOS

### Using homebrew

```sh
brew install beleap/tap/teleproxy
```

### Download from release

1. Download appropriate tar
2. Extarct
3. Move `bin/teleproxy` to somewhere in your `$PATH`
    1. If you get an error "is damaged and can’t be opened." when opening, run following command.
        `xattr -cr <path-to-teleproxy>`

# Starting proxy server

```sh
Usage:
  teleproxy server [flags]

Flags:
  -c, --config string   path for config file
  -h, --help            help for server
  -l, --port int        listening port (default 2344)
  -p, --proxyPort int   proxing port (default 2345)
  -t, --target string   target (default "http://localhost:4000")
```

Default config file location for Docker image is `/etc/teleproxy/config.yaml`
For avaliable values for `config.yaml` see [`examples/config/config.yaml`](../examples/config/config.yaml).
