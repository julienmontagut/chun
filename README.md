<div align="center">

# Chun

--- 

CHeck Unique Name - a command-line tool to check if a name is unique on different registries. 

![GitHub Actions](https://github.com/julienmontagut/chun/actions/workflows/ci.yml/badge.svg)

Available for <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/linux/linux-original.svg" /> and <img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/apple/apple-original.svg" />

---

</div>

## Installation

You can install `chun` with a single command. Chun installs by default to `~/.local/bin` (or `/usr/local/bin` if run as root).

```bash
curl -fsSL https://raw.githubusercontent.com/julienmontagut/chun/main/install.sh | sh
```

## Manual Installation

If you prefer to install manually, you can:
1. Download the latest release for your platform from the [releases page](https://github.com/julienmontagut/chun/releases)
2. Extract the archive: `tar xzf chun-<os>-x86_64.tar.gz`

## Building from Source

Requirements:
- Rust 1.70 or later

```bash
cargo install --git github.com/julienmontagut/chun
``` 

[![Packaging status](https://repology.org/badge/vertical-allrepos/chun.svg?columns=3)](https://repology.org/project/chun/versions)
