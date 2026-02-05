# Editor Setup

Real-time validation in your editor using the agnix LSP server.

## Installation

```bash
cargo install agnix-lsp
```

Or build from source:

```bash
cargo build --release -p agnix-lsp
# Binary at target/release/agnix-lsp
```

## VS Code

Install the extension from source:

```bash
cd editors/vscode
npm install
npm run compile
npm run package
code --install-extension agnix-*.vsix
```

### Settings

```json
{
  "agnix.lspPath": "agnix-lsp",
  "agnix.enable": true,
  "agnix.trace.server": "off"
}
```

### Commands

- `agnix: Restart Language Server` - Restart the server
- `agnix: Show Output Channel` - View debug output

### Troubleshooting

**agnix-lsp not found:**

```bash
# Check if installed
which agnix-lsp  # Unix
where agnix-lsp  # Windows

# Or specify full path in settings
"agnix.lspPath": "/path/to/agnix-lsp"
```

**No diagnostics appearing:**

1. Check file is a supported type (see below)
2. Verify server is running (check status bar)
3. Open output channel for errors

## Neovim

With nvim-lspconfig:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.agnix then
  configs.agnix = {
    default_config = {
      cmd = { 'agnix-lsp' },
      filetypes = { 'markdown', 'json' },
      root_dir = function(fname)
        return lspconfig.util.find_git_ancestor(fname)
      end,
      settings = {},
    },
  }
end

lspconfig.agnix.setup{}
```

## Helix

Add to `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "markdown"
language-servers = ["agnix-lsp"]

[language-server.agnix-lsp]
command = "agnix-lsp"
```

## Supported File Types

- `SKILL.md` - Agent skill definitions
- `CLAUDE.md`, `AGENTS.md` - Memory files
- `.claude/settings.json` - Hook configurations
- `plugin.json` - Plugin manifests
- `*.mcp.json` - MCP tool configurations
- `.github/copilot-instructions.md` - Copilot instructions
- `.cursor/rules/*.mdc` - Cursor project rules

## Features

- Real-time diagnostics as you type
- Quick-fix code actions for auto-fixable issues
- Hover documentation for frontmatter fields
- 100 validation rules
- Status bar indicator (VS Code)
- Syntax highlighting for SKILL.md (VS Code)
