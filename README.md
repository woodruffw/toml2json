toml2json
=========

A command-line tool that converts TOML to JSON. Nothing more, nothing less.

## Installation

```
$ cargo install toml2json
```

Please let us know if you package `toml2json` for another package manager or ecosystem!

## Usage

Convert TOML on `stdin` to JSON, filtering it through `jq`:

```bash
$ toml2json <<< 'wow = "amazing"' | jq
```

Produces:

```json
{
  "wow": "amazing"
}
```

Convert TOML from a file and pretty-print it without `jq`:

```bash
$ toml2json --pretty ~/.config/kbs2/kbs2.conf
```

Produces:

```json
{
  "age-backend": "RageLib",
  "error-hook": "~/.config/kbs2/hooks/error-hook-notify",
  "keyfile": "REDACTED-GO-AWAY",
  "public-key": "REDACTED-GO-AWAY",
  "reentrant-hooks": false,
  "store": "~/.kbs2-store",
  "wrapped": true,
  "generators": [
    {
      "alphabet": "abcdefghijklmnopqrstuvwxyz0123456789(){}[]-_+=",
      "length": 16,
      "name": "default"
    }
  ],
  "commands": {
    "edit": {
      "editor": "subl -w",
      "post-hook": "~/.config/kbs2/hooks/push-repo"
    },
    "new": {
      "generate-on-empty": true,
      "post-hook": "~/.config/kbs2/hooks/push-repo"
    },
    "pass": {
      "clear-after": true,
      "clear-hook": "~/.config/kbs2/hooks/pass-clear-notify",
      "clipboard-duration": 10,
      "x11-clipboard": "Clipboard"
    },
    "rm": {
      "post-hook": "~/.config/kbs2/hooks/push-repo"
    }
  }
}
```

Amazing. What more could you want? Hopefully nothing, because it will never do anything else.
