# AI Coding Assistant Standards Documentation

**Last Updated:** 2026-01-31
**Total Lines:** 8,773+
**Total Size:** 264KB

---

## Quick Navigation

### 🎯 Start Here

**New to cross-platform agent development?**
1. Read: [RESEARCH-SUMMARY.md](RESEARCH-SUMMARY.md) - Executive summary and key findings
2. Reference: [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) - Technical constraints
3. Implement: [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) - Best practices

---

## Document Index

### Multi-Platform Standards (NEW ✨)

| Document | Size | Lines | Purpose |
|----------|------|-------|---------|
| **[multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md)** | 15KB | 455 | Platform compatibility matrix, breaking changes, file naming conventions |
| **[multi-platform-OPINIONS.md](multi-platform-OPINIONS.md)** | 27KB | 973 | Best practices, migration strategies, team workflows |
| **[RESEARCH-SUMMARY.md](RESEARCH-SUMMARY.md)** | 14KB | 395 | Research methodology, sources consulted, key discoveries |

### Claude Code Standards

| Document | Size | Lines | Purpose |
|----------|------|-------|---------|
| **[claude-code-HARD-RULES.md](claude-code-HARD-RULES.md)** | 34KB | 1,150 | Claude Code CLI technical specifications |

### MCP Standards

| Document | Size | Lines | Purpose |
|----------|------|-------|---------|
| **[mcp-HARD-RULES.md](mcp-HARD-RULES.md)** | 33KB | 1,182 | Model Context Protocol specifications |
| **[mcp-OPINIONS.md](mcp-OPINIONS.md)** | 36KB | 1,295 | MCP best practices and patterns |

### Agent Skills Standards

| Document | Size | Lines | Purpose |
|----------|------|-------|---------|
| **[agent-skills-HARD-RULES.md](agent-skills-HARD-RULES.md)** | 19KB | 697 | Skills/prompts technical requirements |
| **[agent-skills-OPINIONS.md](agent-skills-OPINIONS.md)** | 36KB | 1,235 | Skills development best practices |

### Prompt Engineering Standards

| Document | Size | Lines | Purpose |
|----------|------|-------|---------|
| **[prompt-engineering-HARD-RULES.md](prompt-engineering-HARD-RULES.md)** | 16KB | 397 | Prompt engineering constraints |
| **[prompt-engineering-OPINIONS.md](prompt-engineering-OPINIONS.md)** | 21KB | 904 | Prompt engineering best practices |

---

## How to Use This Documentation

### 📖 By Role

**For Developers:**
1. Start with OPINIONS documents for practical guidance
2. Reference HARD-RULES when troubleshooting
3. Check multi-platform docs when integrating new tools

**For Architects:**
1. Read RESEARCH-SUMMARY for strategic overview
2. Use HARD-RULES for technical decision-making
3. Reference OPINIONS for team standards

**For Team Leads:**
1. Review multi-platform-OPINIONS for workflow recommendations
2. Check decision matrices for platform selection
3. Use HARD-RULES to validate compatibility

### 🔍 By Task

**Choosing a Platform:**
→ [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) → Decision Matrix section

**Debugging Compatibility Issues:**
→ [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) → Breaking Incompatibilities section

**Setting Up Team Workflows:**
→ [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) → Team Workflow Recommendations section

**Building MCP Servers:**
→ [mcp-HARD-RULES.md](mcp-HARD-RULES.md) + [mcp-OPINIONS.md](mcp-OPINIONS.md)

**Creating Skills/Prompts:**
→ [agent-skills-HARD-RULES.md](agent-skills-HARD-RULES.md) + [agent-skills-OPINIONS.md](agent-skills-OPINIONS.md)

**Optimizing Prompts:**
→ [prompt-engineering-HARD-RULES.md](prompt-engineering-HARD-RULES.md) + [prompt-engineering-OPINIONS.md](prompt-engineering-OPINIONS.md)

### 🎯 By Platform

**Claude Code:**
- [claude-code-HARD-RULES.md](claude-code-HARD-RULES.md)
- [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) (Claude Code section)

**Cursor:**
- [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) (Cursor section)
- [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) (.cursorrules guidelines)

**Cline:**
- [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) (Cline section)
- [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) (.clinerules guidelines)

**Continue.dev:**
- [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) (Continue.dev section)
- [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) (config.yaml guidelines)

**Aider:**
- [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) (Aider section)
- [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) (.aider.conf.yml guidelines)

**Roo-Cline:**
- [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) (Roo-Cline section)
- [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) (.roomodes guidelines)

---

## Key Findings Summary

### The Reality of Cross-Platform Development

**✅ What Works Across Platforms:**
- MCP (Model Context Protocol) servers
- Environment variables for API keys
- Git repositories
- Markdown documentation
- General coding conventions

**❌ What Doesn't Work Across Platforms:**
- Configuration files (incompatible formats)
- Skills/prompts (different implementations)
- Rules files (platform-specific conventions)
- State directories (proprietary formats)
- Memory files (only Claude Code uses CLAUDE.md)

### Platform File Naming Matrix

| Platform | Memory | Config | Rules | Directory |
|----------|--------|--------|-------|-----------|
| **Claude Code** | CLAUDE.md | CLI args | N/A | .claude/ |
| **Cline** | N/A | .env | .clinerules/ | .cline/ |
| **Roo-Cline** | N/A | .env | N/A | .roo/ |
| **Cursor** | N/A | N/A | .cursorrules | N/A |
| **Continue.dev** | N/A | config.yaml | N/A | .continue/ |
| **Aider** | N/A | .aider.conf.yml | N/A | .aider/ |

### MCP Support Status

| Platform | MCP Support | Transport |
|----------|-------------|-----------|
| Claude Code | ✅ Yes | stdio + HTTP |
| Cline | ✅ Yes | stdio only |
| Continue.dev | ✅ Yes | stdio + HTTP |
| Cursor | ❌ No | N/A |
| Aider | ❌ No | N/A |
| Roo-Cline | ❌ No | N/A |

**Takeaway:** MCP is the ONLY cross-platform standard. Use it for shared functionality.

---

## Common Use Cases

### 1. "I need to support multiple platforms on my team"

**Solution:**
1. Read [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) → "Recommended File Organization"
2. Adopt the `.platform/` directory pattern
3. Build MCP servers for shared functionality
4. Use environment variables for all secrets

**Example Structure:**
```
project-root/
├── .platform/
│   ├── claude-code/
│   ├── cursor/
│   ├── cline/
│   └── continue/
├── .mcp/servers/
└── .env.example
```

### 2. "My configuration broke after a platform update"

**Troubleshooting:**
1. Check [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) → "Breaking Changes"
2. Verify file naming conventions → "File Naming Matrix"
3. Review configuration priority → "Configuration Priority and Override Rules"

### 3. "I want to build a custom skill"

**Implementation:**
1. Define contract: [agent-skills-OPINIONS.md](agent-skills-OPINIONS.md) → "Contract-Based Skills"
2. Check platform capabilities: [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) → "Feature Support Matrix"
3. Implement per-platform: [agent-skills-HARD-RULES.md](agent-skills-HARD-RULES.md)

### 4. "I need to migrate from Cursor to Claude Code"

**Migration Path:**
1. Review [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) → "Platform Migration Issues"
2. Convert .cursorrules → CLAUDE.md conventions
3. Set up .claude/ directory structure
4. Configure environment variables

### 5. "I want to optimize API costs"

**Strategy:**
1. Review [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) → "Performance Optimization"
2. Implement model hierarchy (primary + fallback)
3. Use context window management strategies
4. Configure "weak models" for simple tasks

---

## Research Methodology

### Sources Consulted (15+)

**Official Documentation:**
- Model Context Protocol (modelcontextprotocol.io)
- Aider (aider.chat)
- Continue.dev (docs.continue.dev)
- Cursor (cursor.com)
- Cline (docs.cline.bot)

**GitHub Repositories:**
- Aider-AI/aider
- continuedev/continue
- cline/cline
- RooVetGit/Roo-Cline
- PatrickJS/awesome-cursorrules

**Community Resources:**
- Cursor Community Forum
- GitHub Topics (claude-code, ai-coding-assistant)

**Full research details:** [RESEARCH-SUMMARY.md](RESEARCH-SUMMARY.md)

---

## Document Maintenance

### Update Schedule

**Quarterly (Every 3 months):**
- Check for platform updates
- Verify MCP specification changes
- Review breaking changes

**Annually (Every 12 months):**
- Full research refresh
- Update all matrices and tables
- Validate recommendations

**As-Needed:**
- When team encounters compatibility issues
- When platforms release major versions
- When new standards emerge

### How to Contribute

**Found new information?**
1. Verify with official sources
2. Determine if it's a HARD RULE (factual) or OPINION (recommendation)
3. Update appropriate document
4. Update this README if needed

**HARD RULES:**
- Must be factually verifiable
- Must cite official source
- No subjective language

**OPINIONS:**
- Should include reasoning
- Can reference community patterns
- Should note confidence level

---

## Quick Reference Cards

### Configuration File Cheat Sheet

```bash
# Claude Code
CLAUDE.md                    # Project memory
.claude/skills/              # Custom skills
.claude/hooks/               # Git hooks

# Cursor
.cursorrules                 # AI behavior rules (plain text)

# Cline
.clinerules/                 # Rules directory
.cline/skills/               # Custom skills

# Continue.dev
.continue/config.yaml        # All configuration (YAML)

# Aider
.aider.conf.yml              # Settings (YAML)
.env                         # API keys

# Roo-Cline
.roomodes                    # Custom modes
.roo/                        # State directory
```

### API Key Environment Variables

```bash
# Universal (works everywhere)
ANTHROPIC_API_KEY=sk-ant-...
OPENAI_API_KEY=sk-...

# Aider-specific
AIDER_MODEL=claude-3-5-sonnet-20241022
AIDER_WEAK_MODEL=gpt-4o-mini

# Continue.dev (reference in config.yaml)
${ANTHROPIC_API_KEY}
${OPENAI_API_KEY}
```

### MCP Server Ports

```bash
# Local servers (stdio)
No port needed - uses stdin/stdout

# Remote servers (HTTP)
Configure in platform's MCP settings
Default: varies by server
```

---

## FAQ

**Q: Can I use the same configuration file for Cursor and Cline?**
A: No. Cursor uses `.cursorrules` (plain text file), Cline uses `.clinerules/` (directory). They are incompatible.

**Q: What's the best way to share skills across platforms?**
A: Define skills as contracts (Markdown docs), then implement per-platform. See [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md).

**Q: Is MCP production-ready?**
A: Yes. It's used in Claude Code, Cline, and Continue.dev. See [mcp-HARD-RULES.md](mcp-HARD-RULES.md) for specifications.

**Q: Should I commit my .env file?**
A: NEVER. Commit `.env.example` with placeholder values, but never actual API keys.

**Q: Which platform should my team use?**
A: See [multi-platform-OPINIONS.md](multi-platform-OPINIONS.md) → Decision Matrix section.

**Q: Can I use CLAUDE.md with Cursor?**
A: No. Only Claude Code reads CLAUDE.md. Cursor uses `.cursorrules`.

**Q: How do I migrate from Continue.dev's config.json to config.yaml?**
A: See [multi-platform-HARD-RULES.md](multi-platform-HARD-RULES.md) → Breaking Changes section.

---

## License and Attribution

**Research conducted by:** Claude Code (Anthropic)
**Date:** 2026-01-31
**Status:** Complete ✅

**Sources:** All information derived from publicly available documentation and open-source repositories. See [RESEARCH-SUMMARY.md](RESEARCH-SUMMARY.md) for full attribution.

**Usage:** These documents are intended for internal team use and educational purposes. When sharing externally, please attribute appropriately and link back to source materials.

---

## Contact and Feedback

**Have questions or found errors?**
- Create an issue in your project repository
- Discuss in your team chat
- Update the documentation directly (with proper verification)

**Want to contribute?**
- Follow the "How to Contribute" guidelines above
- Verify all facts with official sources
- Maintain separation between HARD RULES and OPINIONS

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-31 | Initial research and documentation |

---

**Last Updated:** 2026-01-31
**Next Review:** 2026-04-30 (Quarterly)
