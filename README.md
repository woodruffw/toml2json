toml2json
=========

[![CI](https://github.com/woodruffw/toml2json/actions/workflows/ci.yml/badge.svg)](https://github.com/woodruffw/toml2json/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/toml2json)](https://crates.io/crates/toml2json)
![PyPI - Version](https://img.shields.io/pypi/v/toml2json)

A command-line tool that converts TOML to JSON. Nothing more, nothing less.

## Installation

### Cargo

```bash
cargo install toml2json
```

### PyPI

**NOTE**: [toml2json on PyPI](https://pypi.org/project/toml2json/)
was originally a pure Python CLI; ownership of the name on PyPI
was transferred to [woodruffw](https://pypi.org/user/woodruffw/)
for this Rust-based CLI in August 2025. Versions prior to 0.1.0 on PyPI
are the old pure Python version.

```bash
uvx toml2json
uv tool install toml2json
pipx install toml2json

# or pip install, if you're in a virtual environment
```

### Homebrew

`toml2json` is available via [Homebrew](https://brew.sh):

```bash
brew install toml2json
```

### Alpine Linux

`toml2json` is available for [Alpine Edge](https://pkgs.alpinelinux.org/packages?name=toml2json&branch=edge). It can be installed via [apk](https://wiki.alpinelinux.org/wiki/Alpine_Package_Keeper) after enabling the [testing repository](https://wiki.alpinelinux.org/wiki/Repositories).

```bash
apk add toml2json
```

### Arch Linux

If you're using Arch Linux, you can install `toml2json` using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). For example:

```bash
paru -S toml2json
```

### Nixpkgs

```
nix-env --install toml2json
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
