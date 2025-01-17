<div align="center">

# Chun

--- 

CHeck Unique Name - a command-line tool to check if a name is unique on different registries. 

![GitHub Actions](https://github.com/julienmontagut/chun/actions/workflows/ci.yml/badge.svg)

Available for <i class="devicon-linux-plain"></i><i class="devicon-apple-original"></i>

</div>

## Installation

You can install `chun` with a single command. Chun installs by default to `~/.local/bin` (or `/usr/local/bin` if run as root).

```bash
curl -fsSL https://raw.githubusercontent.com/julienmontagut/chun/main/install.sh | sh
```

This will:
1. Detect your operating system (Linux or macOS)
2. Download the latest release
3. Install it to `~/.local/bin` (if run as user) or `/usr/local/bin` (if run as root)

## Manual Installation

If you prefer to install manually, you can:
1. Download the latest release for your platform from the [releases page](https://github.com/julienmontagut/chun/releases)
2. Extract the archive: `tar xzf chun-<os>-x86_64.tar.gz`
3. Move the binary to your PATH: `mv chun ~/.local/bin/` or `sudo mv chun /usr/local/bin/`

## Building from Source

Requirements:
- Rust 1.70 or later

```bash
cargo install --git github.com/julienmontagut/chun
``` 

[![Packaging status](https://repology.org/badge/vertical-allrepos/chun.svg?columns=3)](https://repology.org/project/chun/versions)

<link rel="stylesheet" type='text/css' href="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" />