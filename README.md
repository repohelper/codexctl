# PolyCLI

[![CI](https://github.com/repohelper/polycli/actions/workflows/ci.yml/badge.svg)](https://github.com/repohelper/polycli/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.94%2B-blue.svg)](https://www.rust-lang.org)

> **Universal AI CLI Profile Manager** - Manage multiple AI CLI accounts (Codex, Claude Code, Gemini CLI, and more)

**Version**: 0.1.0 | **Author**: [Bhanu Korthiwada](https://github.com/BhanuKorthiwada) | **Status**: ✅ Public Beta

🔗 **Website**: [polycli.repohelper.com](https://polycli.repohelper.com)  
📖 **Documentation**: [polycli.repohelper.com/docs](https://polycli.repohelper.com/docs)

---

## Why PolyCLI?

If you work with multiple AI CLI tools (OpenAI Codex, Claude Code, Gemini CLI) and have multiple accounts (work, personal, side projects), PolyCLI lets you:

- 🔐 **Securely store** multiple AI CLI profiles with optional encryption
- ⚡ **Switch instantly** between accounts without re-authenticating
- 🤖 **Auto-switch** based on quota availability
- 📊 **Monitor usage** across all your AI CLI accounts
- 🌳 **Use concurrently** - different accounts in different terminals

---

## Features

### Core
- ✅ **Multi-CLI Support** - Works with Codex CLI, Claude Code, Gemini CLI (extensible)
- 🔐 **Optional Encryption** - age-based encryption for sensitive auth data
- 🚀 **Fast Switching** - Switch accounts in < 1 second
- 🔄 **Quick-Switch** - Toggle between current and previous profile with `poly -`
- 🗂️ **Profile Management** - Save, load, list, delete, backup profiles

### Advanced
- 🤖 **Auto-Switcher** - Automatically pick the best profile based on quota availability
- 📊 **Real-Time Quota** - Live usage data from AI provider APIs
- ✅ **Verify Command** - Validate all profiles' authentication status
- 🌳 **Concurrent Usage** - Use multiple profiles simultaneously via `env` command
- 📦 **Import/Export** - Transfer profiles between machines securely

### Developer Experience
- 🖥️ **Cross-Platform** - macOS, Linux, Windows support
- 🔧 **Shell Completions** - Bash, Zsh, Fish, PowerShell
- 🧪 **Zero Dependencies** - Single binary, no runtime requirements
- 🐳 **Docker Support** - Multi-arch images available
- 🧬 **Auto-Migration** - Seamless upgrades between versions

---

## Quick Start

### Install

```bash
# Via cargo
cargo install polycli

# Or download binary
curl -fsSL https://polycli.repohelper.com/install.sh | sh

# Or via Homebrew (macOS/Linux)
brew install repohelper/tap/polycli
```

### First Steps

```bash
# Save your current AI CLI profile
poly save work

# Create another profile
# (switch accounts in your AI CLI, then:)
poly save personal

# List all profiles
poly list

# Switch to a profile
poly load work

# Quick-switch to previous profile
poly load -
```

---

## Supported AI CLIs

| CLI | Status | Command |
|-----|--------|---------|
| OpenAI Codex CLI | ✅ Full Support | `poly --cli codex save work` |
| Claude Code | ✅ Full Support | `poly --cli claude save work` |
| Gemini CLI | ✅ Full Support | `poly --cli gemini save work` |
| OpenAI CLI | ✅ Full Support | `poly --cli openai save work` |

---

## Commands

```
poly save <name>              Save current AI CLI auth as a profile
poly load <name>              Load a saved profile and switch to it
poly list                     List all saved profiles
poly delete <name>            Delete a saved profile
poly status                   Show current profile status
poly usage                    Show usage limits and subscription info
poly verify                   Verify all profiles' authentication status
poly backup                   Create a backup of current profile
poly run <name> -- <cmd>      Run a command with a specific profile
poly env <name>               Export shell commands for concurrent usage
poly diff <name1> <name2>     Compare/diff two profiles
poly switch                   Switch to a profile interactively (fzf)
poly history                  View command history
poly doctor                   Run health check on profiles
poly completions              Generate shell completions
poly import <file>            Import a profile from another machine
poly export <name>            Export a profile for transfer
poly setup                    Interactive setup wizard
```

---

## Encryption (Optional)

```bash
# Save with encryption
poly save work --passphrase "my-secret"

# Or use environment variable
export POLY_PASSPHRASE="my-secret"
poly save work

# Load encrypted profile
poly load work --passphrase "my-secret"
```

---

## Auto-Switcher

Let PolyCLI pick the best profile automatically:

```bash
# Switch to profile with most quota available
poly load auto

# Configure auto-switch preferences
poly config set auto_switch.threshold 80
poly config set auto_switch.prefer work,personal
```

---

## Shell Integration

Add to your `.bashrc`/`.zshrc`:

```bash
# Enable completions
source <(poly completions bash)

# Optional: Auto-switch based on directory (like direnv)
eval "$(poly init --shell zsh)"
```

---

## Docker

```bash
# Run with Docker
docker run -it --rm \
  -v ~/.polycli:/root/.config/polycli \
  -v ~/.codex:/root/.codex \
  ghcr.io/repohelper/polycli list
```

---

## Configuration

Configuration directory: `~/.config/polycli/`

```toml
# ~/.config/polycli/config.toml
[default]
cli = "codex"  # Default AI CLI to manage

[auto_switch]
enabled = true
threshold = 80  # Switch when quota below 80%

[encryption]
default_passphrase = false  # Always prompt for passphrase
```

---

## Comparison

| Feature | PolyCLI | [codex-profiles](https://github.com/midhunmonachan/codex-profiles) | [aisw](https://github.com/burakdede/aisw) |
|---------|---------|-------------------------------------------------------------------|-------------------------------------------|
| Multi-CLI | ✅ | ❌ | ✅ |
| Encryption | ✅ | ❌ | ❌ |
| Auto-Switcher | ✅ | ❌ | ❌ |
| Real-Time Quota | ✅ | ❌ | ❌ |
| Shell Completions | ✅ | ❌ | ❌ |
| Docker Support | ✅ | ❌ | ❌ |
| Cross-Platform | ✅ | ✅ | ✅ |

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](./LICENSE) for details.

---

**Made with ❤️ by [Bhanu Korthiwada](https://github.com/BhanuKorthiwada)**  
Part of the [RepoHelper](https://repohelper.com) project collection.
