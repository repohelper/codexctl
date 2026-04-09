# PolyCLI

Universal AI CLI Profile Manager - Manage multiple AI CLI accounts (Codex, Claude, Gemini, OpenAI)

## Installation

```bash
npm install -g polycli
```

Or use npx (no install):
```bash
npx polycli --help
```

## Usage

```bash
# Save your current AI CLI profile
poly codex save work
poly claude save personal

# Switch between profiles
poly codex load work
poly claude load personal

# List all profiles
poly list

# Quick-switch to previous profile
poly load -

# Auto-switch to best profile based on quota
poly load auto
```

## Supported AI CLIs

- **Codex CLI** (`poly codex`)
- **Claude Code** (`poly claude`)
- **Gemini CLI** (`poly gemini`)
- **OpenAI CLI** (`poly openai`)

## Features

- 🔐 **Optional Encryption** - age-based encryption for sensitive auth data
- 🚀 **Fast Switching** - Switch accounts in < 1 second
- 🤖 **Auto-Switcher** - Automatically pick the best profile based on quota
- 📊 **Real-Time Quota** - Live usage data from AI provider APIs
- 🌳 **Concurrent Usage** - Use multiple profiles simultaneously

## Documentation

Full documentation: https://polycli.repohelper.com

## License

MIT
