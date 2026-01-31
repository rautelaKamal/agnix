# Multi-Platform Agent Standards - HARD RULES

## Document Purpose
This document contains FACTUAL, NON-NEGOTIABLE compatibility rules for cross-platform AI coding assistant projects. These are technical constraints, not opinions.

**Last Updated:** 2026-01-31
**Sources:** 15+ official documentation sites, repositories, and community forums

---

## File Naming (Platform Matrix)

| Platform | Memory File | Config File | Rules File | Directory | State Dir |
|----------|-------------|-------------|------------|-----------|-----------|
| **Claude Code** | CLAUDE.md | N/A (uses CLI args) | N/A | .claude/ | .claude/ |
| **Cline** | N/A | .env | .clinerules/ | .cline/ | .cline/ |
| **Roo-Cline** | N/A | .env | N/A | .roo/ | .roo/ |
| **Cursor** | N/A | N/A | .cursorrules | N/A | N/A |
| **Continue.dev** | N/A | config.yaml | N/A | .continue/ | .continue/ |
| **Aider** | N/A | .aider.conf.yml | N/A | N/A | .aider/ |

### Critical File Naming Rules

1. **CLAUDE.md vs AGENTS.md**
   - `CLAUDE.md` - Used by Claude Code (Anthropic's official CLI)
   - `AGENTS.md` - NO OFFICIAL PLATFORM USES THIS (community convention only)
   - **BREAKING:** Using AGENTS.md with Claude Code will NOT be recognized

2. **Configuration File Extensions**
   - Aider: **MUST** use `.aider.conf.yml` (YAML only)
   - Continue.dev: **MUST** use `config.yaml` (JSON deprecated as of 2025)
   - Cursor: **MUST** use `.cursorrules` (no extension variant)
   - Cline/Roo-Cline: **MUST** use `.env` for secrets

3. **Rules File Conventions**
   - Cursor: `.cursorrules` - plain text file in project root
   - Cline: `.clinerules/` - directory structure (not a file)
   - **BREAKING:** `.cursorrules` will NOT work in Cline (expects directory)
   - **BREAKING:** `.clinerules/` will NOT work in Cursor (expects file)

---

## Directory Structure Differences

### State Directory Patterns

```
Claude Code (official):
.claude/
├── skills/           # Custom skills
├── hooks/            # Git hooks
└── state/            # Session state

Cline:
.cline/
└── skills/           # Custom skills only
    └── create-pull-request/

Roo-Cline:
.roo/
└── (unspecified structure)

Continue.dev:
.continue/
└── config.yaml       # All config in one file

Aider:
.aider/               # State only (no config here)
```

### Configuration File Locations

| Platform | Home Directory | Project Root | Custom Path |
|----------|----------------|--------------|-------------|
| **Aider** | `~/.aider.conf.yml` | `.aider.conf.yml` | `--config <path>` |
| **Continue.dev** | `~/.continue/config.yaml` | `.continue/config.yaml` | N/A |
| **Cline** | N/A | `.env` | Via VSCode settings |
| **Cursor** | N/A | `.cursorrules` | N/A |

**BREAKING:** Aider loads configs sequentially (home → git root → cwd), with later files overriding earlier ones. Continue.dev uses ONLY ONE config file location per context.

---

## Feature Support Matrix

| Feature | Claude Code | Cline | Roo-Cline | Cursor | Continue.dev | Aider |
|---------|-------------|-------|-----------|--------|--------------|-------|
| **Hooks** | Yes | No | No | No | No | No |
| **Skills** | Yes | Yes | Yes | Partial (context rules) | Yes (prompts) | No |
| **MCP Servers** | Yes | Yes | No | No | Yes | No |
| **Custom Modes** | No | No | Yes | No | No | No |
| **Voice Input** | No | No | No | No | No | Yes |
| **Browser UI** | No | No | No | No | Yes (headless) | Yes |
| **CLI Interface** | Yes | Yes (experimental) | No | No | Yes | Yes |
| **IDE Extensions** | No | Yes (VSCode) | Yes (VSCode) | Built-in editor | Yes (VSCode, JetBrains) | No (watch mode) |

### Model Context Protocol (MCP) Support

**MCP is the ONLY standardized cross-platform protocol for AI coding assistants.**

| Platform | MCP Support | MCP Version | Transport |
|----------|-------------|-------------|-----------|
| Claude Code | Yes | Latest | stdio, HTTP |
| Cline | Yes | Latest | stdio |
| Continue.dev | Yes | Latest | stdio, HTTP |
| Cursor | No | N/A | N/A |
| Aider | No | N/A | N/A |
| Roo-Cline | No | N/A | N/A |

**CRITICAL:** MCP is the only protocol with official specifications. All other "standards" are platform-specific conventions.

---

## Breaking Incompatibilities

### 1. Configuration Format Conflicts

**Continue.dev JSON → YAML Migration (2025)**
- `config.json` is **DEPRECATED**
- **BREAKING:** Old configs will NOT work in new versions
- Migration required: Manual conversion to `config.yaml`

**Aider Environment Variables**
- Convention: `AIDER_<OPTION>` (uppercase, underscores)
- **BREAKING:** Generic `AI_*` vars will NOT work
- Example: `AIDER_DARK_MODE=true` (NOT `AI_DARK_MODE`)

### 2. Skills/Prompts Incompatibility

**Skills Directory Structure:**
```
Claude Code: .claude/skills/<skill-name>/SKILL.md
Cline:       .cline/skills/<skill-name>/
Roo-Cline:   Custom modes in .roomodes file
Continue:    Prompts in config.yaml
```

**BREAKING:** Cannot share skill directories between platforms. Each uses different:
- File formats (SKILL.md vs config.yaml vs .roomodes)
- Discovery mechanisms
- Execution contexts

### 3. Rules File Format Differences

**Cursor (.cursorrules):**
```
Plain text instructions
No special formatting required
Lives in project root
```

**Cline (.clinerules/):**
```
Directory-based structure
Multiple rule files possible
Subdirectory organization
```

**BREAKING:** A `.cursorrules` file will be IGNORED by Cline. A `.clinerules/` directory will be IGNORED by Cursor.

### 4. Memory/Context File Incompatibility

**Claude Code:**
- Uses `CLAUDE.md` for project memory
- Parsed by CLI on every session start
- Supports frontmatter and structured sections

**Other Platforms:**
- NO standardized memory file format
- Each platform has proprietary context management
- **BREAKING:** `CLAUDE.md` has NO effect in Cursor, Cline, Continue, or Aider

### 5. Transport Protocol Limitations

**MCP Transport Support:**

| Transport | Claude Code | Cline | Continue.dev |
|-----------|-------------|-------|--------------|
| stdio (local) | Yes | Yes | Yes |
| HTTP (remote) | Yes | No | Yes |
| SSE (streaming) | Yes | No | Yes |

**BREAKING:** Remote MCP servers over HTTP will NOT work with Cline (stdio only).

---

## Configuration Priority and Override Rules

### Aider Configuration Hierarchy (Loading Order)
1. `~/.aider.conf.yml` (lowest priority)
2. `<git-root>/.aider.conf.yml`
3. `./.aider.conf.yml` (highest priority - current directory)
4. Environment variables (`AIDER_*`)
5. `.env` file
6. Command-line arguments (override all)

**BREAKING:** Settings in later files completely override earlier ones (no merging).

### Continue.dev Configuration Loading
- Loads ONLY ONE config file per context
- No cascading/merging between home and project configs
- **BREAKING:** Cannot use global config + project overrides like Aider

### Cursor Configuration
- `.cursorrules` is the ONLY configuration mechanism
- No cascading, no environment variables, no config files
- **BREAKING:** Cannot split rules across multiple files or locations

---

## API Key and Secrets Management

### Platform-Specific Requirements

| Platform | Primary Method | Fallback Methods | Security Notes |
|----------|----------------|------------------|----------------|
| **Aider** | `.env` file | `AIDER_*` env vars | NO API keys in YAML (security risk) |
| **Claude Code** | Environment vars | N/A | NO built-in secrets management |
| **Cline** | `.env` file | VSCode settings | Supports multiple providers |
| **Continue.dev** | `config.yaml` | Environment vars | Keys in config (less secure) |
| **Cursor** | Built-in UI | N/A | Stored in app settings |

**CRITICAL SECURITY RULE:** Aider documentation explicitly warns: "only put OpenAI and Anthropic API keys in the YAML config file." Other keys MUST go in `.env`.

---

## State Management Differences

### Session State Persistence

| Platform | State Storage | Persistence | Session Resume |
|----------|---------------|-------------|----------------|
| Claude Code | `.claude/state/` | Yes | Yes |
| Cline | `.cline/` | Partial | No |
| Continue.dev | `.continue/` | Yes | Yes |
| Aider | `.aider/` | Yes | Yes (via git) |
| Cursor | Cloud-based | Yes | Yes |
| Roo-Cline | `.roo/` | Partial | No |

**BREAKING:** State files are NOT portable between platforms. Each uses proprietary serialization formats.

---

## Testing and Build Configurations

### Platform Testing Standards

| Platform | Test Framework | Config File | E2E Testing |
|----------|----------------|-------------|-------------|
| Cline | Mocha + Playwright | `.mocharc.json`, `playwright.config.ts` | Yes |
| Continue.dev | (unspecified) | N/A | Unknown |
| Roo-Cline | (unspecified) | N/A | Unknown |
| Aider | pytest | `pytest.ini` | Yes |

### Build Systems

| Platform | Language | Build Tool | Config File |
|----------|----------|------------|-------------|
| Cline | TypeScript (86.6%) | esbuild | `esbuild.mjs` |
| Continue.dev | TypeScript (84.1%) | (unspecified) | `tsconfig.json` |
| Roo-Cline | TypeScript (98.6%) | Turbo | `turbo.json` |
| Aider | Python | setuptools | `pyproject.toml` |

---

## Package Manager and Dependency Management

### Required Package Managers

| Platform | Package Manager | Workspace Support | Monorepo |
|----------|----------------|-------------------|----------|
| Cline | npm/yarn | No | No |
| Continue.dev | npm/yarn | Yes | Yes |
| Roo-Cline | **pnpm ONLY** | Yes | Yes |
| Aider | pip | N/A | N/A |

**BREAKING:** Roo-Cline explicitly requires `pnpm`. Using npm or yarn will NOT work with their workspace configuration (`pnpm-workspace.yaml`).

---

## Localization and Internationalization

### Multi-Language Support

| Platform | i18n Support | Locales Dir | Languages |
|----------|--------------|-------------|-----------|
| Cline | Yes | `locales/` | Multiple |
| Roo-Cline | Yes | `locales/` | 18+ languages |
| Others | No | N/A | English only |

**BREAKING:** Localization files are platform-specific. Cannot share locale files between Cline and Roo-Cline.

---

## Git Integration Patterns

### Auto-Commit Behavior

| Platform | Auto-Commit | Commit Messages | Undo Support |
|----------|-------------|-----------------|--------------|
| Aider | Yes (default) | Auto-generated | Yes (`/undo`) |
| Claude Code | Opt-in (hooks) | Template-based | Via git |
| Others | No | N/A | Manual only |

**BREAKING:** Aider auto-commits ALL changes by default. Other platforms require manual commits.

---

## Version Control File Patterns

### Common .gitignore Patterns

```gitignore
# Platform-specific directories (ADD TO .gitignore)
.claude/state/
.cline/
.roo/
.continue/
.aider/

# Secrets and API keys (CRITICAL - ALWAYS IGNORE)
.env
.env.local
.aider.conf.yml  # If it contains API keys

# Platform-specific state
.cursorrules      # Project-specific, COMMIT THIS
config.yaml       # Project-specific Continue config, COMMIT THIS
.aider.conf.yml   # Project-specific settings, COMMIT THIS (no keys)
```

**SECURITY CRITICAL:** Never commit API keys. Aider docs explicitly warn against putting keys in YAML files.

---

## Cross-Platform Compatibility Summary

### What CAN Be Shared

1. **General coding conventions** (language-agnostic)
2. **Documentation in standard formats** (Markdown, etc.)
3. **MCP server configurations** (Claude Code, Cline, Continue.dev only)
4. **Git repository** (all platforms use git)

### What CANNOT Be Shared

1. **Skills/prompts** (different formats per platform)
2. **Configuration files** (incompatible schemas)
3. **Rules files** (different conventions)
4. **State directories** (proprietary formats)
5. **Memory files** (only Claude Code uses CLAUDE.md)

---

## Platform Migration Issues

### Migrating FROM → TO

**From Cursor to Claude Code:**
- **BREAKING:** `.cursorrules` → No equivalent. Use CLI arguments or CLAUDE.md conventions.
- **BREAKING:** No direct migration path for rules.

**From Aider to Continue.dev:**
- **BREAKING:** `.aider.conf.yml` → `config.yaml` (completely different schemas)
- Must manually recreate model configurations

**From Continue.dev (old) to Continue.dev (new):**
- **BREAKING:** `config.json` → `config.yaml` (deprecated, manual migration required)

**From Cline to Roo-Cline:**
- **PARTIAL:** `.cline/` → `.roo/` (similar structure, but modes differ)
- Skills may require adaptation

---

## Known Bugs and Limitations

### Platform-Specific Bugs

**Aider:**
- **BUG:** Never use `-uall` flag with git status (memory issues on large repos)

**Continue.dev:**
- **DEPRECATED:** config.json support removed in 2025
- **LIMITATION:** Hub configurations vs. Local configurations are mutually exclusive

**Cursor:**
- **LIMITATION:** No programmatic access to settings
- **LIMITATION:** No CLI interface

**Cline:**
- **LIMITATION:** CLI is experimental
- **LIMITATION:** MCP supports stdio only (no HTTP)

---

## Future-Proofing Recommendations

### What to Expect (Based on Current Trends)

1. **MCP Adoption:** Expect more platforms to adopt MCP as the standard protocol
2. **Config Consolidation:** Continue.dev's YAML migration suggests industry trend toward YAML
3. **IDE Integration:** More platforms moving toward VSCode/JetBrains extensions
4. **Remote MCP Servers:** HTTP transport becoming more common

### What to Avoid

1. **DO NOT** build abstractions assuming all platforms support the same features
2. **DO NOT** assume file naming conventions will converge (they haven't in 3+ years)
3. **DO NOT** expect backward compatibility (see Continue.dev config.json deprecation)
4. **DO NOT** share state directories between platforms (corruption risk)

---

## Verification Methods

### How to Test Cross-Platform Compatibility

1. **Skills/Prompts:** Test in each platform independently (no cross-platform testing possible)
2. **MCP Servers:** Test stdio transport (most compatible)
3. **Configuration:** Validate each platform's config schema separately
4. **API Keys:** Use environment variables (most portable method)

---

## References and Sources

### Official Documentation
- Model Context Protocol: https://modelcontextprotocol.io
- Aider: https://aider.chat/docs
- Continue.dev: https://docs.continue.dev
- Cline: https://docs.cline.bot
- Cursor: https://cursor.com/docs

### Community Resources
- awesome-cursorrules: https://github.com/PatrickJS/awesome-cursorrules
- Cline GitHub: https://github.com/cline/cline
- Continue GitHub: https://github.com/continuedev/continue
- Aider GitHub: https://github.com/Aider-AI/aider
- Roo-Cline GitHub: https://github.com/RooVetGit/Roo-Cline

### Research Date
Data collected: 2026-01-31
Total sources consulted: 15+

---

## Document Maintenance

**This is a HARD RULES document.** Only add information that is:
1. Factually verifiable from official sources
2. Non-negotiable technical constraints
3. Breaking changes or incompatibilities

For opinions, best practices, or recommendations, use the companion document: `multi-platform-OPINIONS.md`
