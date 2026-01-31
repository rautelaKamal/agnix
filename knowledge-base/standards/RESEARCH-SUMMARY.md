# Cross-Platform Agent Standards Research Summary

**Research Date:** 2026-01-31  
**Sources Consulted:** 15+  
**Documents Generated:** 2

---

## Executive Summary

Research conducted across 15+ sources including official documentation, GitHub repositories, and community forums to establish factual technical constraints and best practices for cross-platform AI coding assistant development.

### Key Findings

1. **NO universal standard exists** except Model Context Protocol (MCP)
2. **File naming conventions differ significantly** across platforms
3. **Skills/prompts are NOT portable** between platforms
4. **Configuration formats are incompatible** (YAML, JSON, plain text)
5. **MCP is the only viable cross-platform abstraction** for shared functionality

---

## Sources Consulted

### Official Documentation
1. **Model Context Protocol** - https://modelcontextprotocol.io
   - Architecture and specifications
   - Client/server conventions
   - Transport protocols (stdio, HTTP)

2. **Aider** - https://aider.chat/docs
   - Configuration file format (.aider.conf.yml)
   - Environment variable conventions
   - Git integration patterns

3. **Continue.dev** - https://docs.continue.dev
   - Config migration (JSON → YAML)
   - config.yaml schema
   - MCP integration

4. **Cursor** - https://cursor.com/docs
   - .cursorrules conventions
   - Context management

5. **Cline** - https://docs.cline.bot
   - .clinerules/ directory structure
   - .cline/ state management
   - MCP support

### GitHub Repositories
6. **Aider-AI/aider** - https://github.com/Aider-AI/aider
   - Python implementation
   - Testing patterns (pytest)
   - Repository structure

7. **continuedev/continue** - https://github.com/continuedev/continue
   - TypeScript implementation (84.1%)
   - Monorepo structure
   - .continueignore patterns

8. **cline/cline** - https://github.com/cline/cline
   - VS Code extension architecture
   - Skills directory pattern
   - Testing frameworks (Mocha, Playwright)

9. **RooVetGit/Roo-Cline** - https://github.com/RooVetGit/Roo-Cline
   - Custom modes (.roomodes)
   - pnpm workspace configuration
   - 18+ language localization

10. **PatrickJS/awesome-cursorrules** - https://github.com/PatrickJS/awesome-cursorrules
    - .cursorrules examples
    - Community conventions
    - Cross-framework patterns

### Community Resources
11. **Cursor Community Forum** - https://forum.cursor.com
    - User discussions
    - Configuration best practices
    - Cross-platform discussions

12. **Anthropic Tools (Deprecated)** - https://github.com/anthropics/anthropic-tools
    - Legacy tool use patterns
    - BaseTool and ToolUser classes
    - Message format conventions

### API Documentation
13. **GitHub Copilot** - https://github.com/features/copilot
    - Platform support matrix
    - IDE integration patterns

14. **OpenAI Codex** - https://openai.com/index/openai-codex
    - (Deprecated/limited information)

15. **MCP Reference Servers** - https://github.com/modelcontextprotocol/servers
    - Implementation examples
    - Best practices

---

## Research Methodology

### Data Collection Process

1. **Primary Sources (Official Docs)**
   - Fetched documentation from official sites
   - Extracted technical specifications
   - Identified explicit constraints and requirements

2. **Secondary Sources (GitHub Repos)**
   - Analyzed repository structures
   - Examined configuration files
   - Reviewed testing and build patterns

3. **Tertiary Sources (Community)**
   - Surveyed forum discussions
   - Identified common patterns
   - Gathered real-world usage insights

### Information Verification

- Cross-referenced claims across multiple sources
- Prioritized official documentation over community opinions
- Separated HARD RULES (factual constraints) from OPINIONS (best practices)
- Documented source attribution for all technical claims

---

## Key Discoveries

### 1. Platform File Naming Matrix

| Platform | Memory | Config | Rules | Directory |
|----------|--------|--------|-------|-----------|
| Claude Code | CLAUDE.md | CLI args | N/A | .claude/ |
| Cline | N/A | .env | .clinerules/ | .cline/ |
| Roo-Cline | N/A | .env | N/A | .roo/ |
| Cursor | N/A | N/A | .cursorrules | N/A |
| Continue.dev | N/A | config.yaml | N/A | .continue/ |
| Aider | N/A | .aider.conf.yml | N/A | .aider/ |

**Impact:** No file naming standard exists. Each platform requires unique file names.

### 2. Configuration Format Incompatibility

**Continue.dev:**
- Deprecated: config.json (as of 2025)
- Current: config.yaml
- Schema: YAML 1.1 with anchors

**Aider:**
- Format: .aider.conf.yml (YAML only)
- Supports: Cascading configs (home → project → cwd)
- Environment: AIDER_* variables

**Cursor:**
- Format: .cursorrules (plain text)
- Location: Project root only
- No cascading or inheritance

**Impact:** Cannot share configuration files across platforms.

### 3. Skills/Prompts Portability: IMPOSSIBLE

**Claude Code:**
```
.claude/skills/<name>/SKILL.md
```

**Cline:**
```
.cline/skills/<name>/
```

**Roo-Cline:**
```
.roomodes (custom modes in single file)
```

**Continue.dev:**
```yaml
prompts:
  - name: "skill-name"
    prompt: "..."
```

**Impact:** Skills must be reimplemented per-platform using different formats.

### 4. MCP: The Only Cross-Platform Standard

**Platforms with MCP Support:**
- ✅ Claude Code (stdio + HTTP)
- ✅ Cline (stdio only)
- ✅ Continue.dev (stdio + HTTP)
- ❌ Cursor
- ❌ Aider
- ❌ Roo-Cline

**Impact:** MCP servers work across 3 major platforms TODAY. This is the only portable abstraction.

### 5. Testing and Build Systems

**TypeScript Platforms:**
- Cline: esbuild, Mocha, Playwright
- Continue.dev: (unspecified testing)
- Roo-Cline: Turbo (monorepo), pnpm workspaces

**Python Platforms:**
- Aider: pytest, setuptools, pyproject.toml

**Impact:** No shared testing conventions. Each platform has unique build requirements.

### 6. API Key Management Patterns

**Best Practice (Aider docs):**
> "Only put OpenAI and Anthropic API keys in the YAML config file"
> Other keys should go in .env files

**Platforms:**
- Aider: .env preferred, AIDER_* env vars
- Cline: .env required
- Continue.dev: config.yaml (less secure)
- Cursor: Built-in UI

**Impact:** Environment variables are the most portable secret management approach.

---

## Notable Absences

### What We Did NOT Find

1. **AGENTS.md Convention**
   - Searched: GitHub, documentation sites
   - Result: NO official platform uses this name
   - Conclusion: Community convention only, not standardized

2. **OpenCode Platform**
   - Searched: opencode.ai, GitHub
   - Result: No substantial documentation found
   - Status: Either deprecated or very limited adoption

3. **Codex (OpenAI)**
   - Searched: OpenAI documentation
   - Result: Limited/deprecated information
   - Status: Superseded by ChatGPT/GPT-4 integrations

4. **Cross-Platform Config Standards**
   - Searched: All sources
   - Result: NO emerging standards beyond MCP
   - Conclusion: Ecosystem remains fragmented

---

## Breaking Changes Documented

### Continue.dev (2025)
- **BREAKING:** config.json → config.yaml migration
- **Impact:** All users must manually migrate
- **Reason:** YAML supports anchors and better structure

### Anthropic Tools (2024)
- **BREAKING:** anthropic-tools repository deprecated
- **Impact:** Tool use patterns moved to official API docs
- **Reason:** Consolidation of documentation

### Aider Git Operations
- **BREAKING:** -uall flag causes memory issues on large repos
- **Impact:** Must avoid this flag in automation
- **Reason:** Performance bug

---

## Recommendations Derived from Research

### For Individual Developers
1. Choose ONE platform based on your workflow preferences
2. Invest in learning that platform deeply
3. Use MCP for any custom integrations (future-proof)

### For Teams
1. Support 2-3 platforms maximum (balance flexibility vs. overhead)
2. Use `.platform/` directory pattern for organization
3. Build MCP servers for shared functionality
4. Use environment variables for ALL secrets

### For Tool Builders
1. Adopt MCP for maximum compatibility
2. Provide clear migration paths when deprecating features
3. Document configuration schemas explicitly
4. Support environment variable configuration

---

## Future Research Areas

### Questions Remaining

1. **Cursor .cursorrules Specification**
   - No formal schema found
   - Community examples exist but no official docs
   - Need: Official specification from Cursor team

2. **Cline .clinerules/ Structure**
   - Directory-based, but no schema documented
   - Need: Official file format specification

3. **MCP Adoption Timeline**
   - When will Cursor/Aider/Roo-Cline support MCP?
   - Need: Roadmaps from platform teams

4. **Emerging Standards**
   - Will any new cross-platform standards emerge?
   - Need: Monitor industry developments

### Recommended Monitoring

- Track MCP specification updates (modelcontextprotocol.io)
- Watch for configuration migrations in major platforms
- Monitor community discussions for emerging patterns
- Subscribe to platform release notes

---

## Document Maintenance

### Update Triggers

This research should be updated when:
1. Major platform releases change configuration standards
2. New cross-platform standards emerge
3. Platforms add/remove MCP support
4. Configuration formats are deprecated/migrated

### Review Schedule

- **Quarterly:** Check for platform updates
- **Annually:** Full research refresh
- **As-needed:** When team encounters compatibility issues

---

## Files Generated

### 1. multi-platform-HARD-RULES.md
**Size:** 455 lines, 15KB  
**Content:** Factual technical constraints, breaking changes, compatibility matrices

**Key Sections:**
- File naming matrix (6 platforms)
- Feature support matrix
- Breaking incompatibilities
- Configuration priority rules
- API key management
- State directory patterns

**Audience:** Technical decision-makers, platform integrators

### 2. multi-platform-OPINIONS.md
**Size:** 973 lines, 27KB  
**Content:** Best practices, recommendations, strategic guidance

**Key Sections:**
- When to use platform-specific features
- Recommended file organization patterns
- Configuration strategies
- Cross-platform skills/prompts approach
- MCP investment recommendations
- Migration strategies
- Team workflow recommendations
- Security best practices
- Performance optimization
- Platform-specific power tips
- Decision matrices

**Audience:** Developers, team leads, architects

---

## Usage Guidelines

### For Quick Reference
- Use HARD-RULES.md to verify compatibility
- Check feature support matrix before choosing platforms
- Reference breaking incompatibilities during migrations

### For Strategic Planning
- Use OPINIONS.md for team discussions
- Reference decision matrices when choosing platforms
- Follow recommended patterns for new projects

### For Troubleshooting
- Check HARD-RULES for known incompatibilities
- Reference API key management section
- Review state management differences

---

## Confidence Levels

### HIGH CONFIDENCE (Verified in 3+ sources)
- File naming conventions
- MCP support status
- Configuration format differences
- Breaking changes (Continue.dev, Anthropic Tools)

### MEDIUM CONFIDENCE (Verified in 1-2 sources)
- Some feature availability (voice, browser UI)
- Testing framework details
- Build system specifics

### LOW CONFIDENCE (Community sources only)
- Future roadmaps
- Undocumented features
- Emerging patterns

---

## Limitations of This Research

### Scope Limitations
1. **No hands-on testing** - Documentation review only
2. **Point-in-time snapshot** - Standards evolve rapidly
3. **English sources only** - May miss non-English documentation
4. **Public information only** - No access to private/enterprise docs

### Known Gaps
1. **Cursor internal architecture** - Limited public documentation
2. **OpenCode status** - Insufficient information found
3. **Enterprise features** - Consumer-focused research
4. **Performance benchmarks** - No comparative testing

### Mitigation Strategies
- Cross-referenced multiple sources where possible
- Prioritized official documentation
- Clearly separated facts from opinions
- Documented confidence levels

---

## Conclusion

The AI coding assistant ecosystem remains **highly fragmented** with **NO universal standards** except Model Context Protocol (MCP). Each platform has unique:
- File naming conventions
- Configuration formats
- Skills/prompts systems
- State management approaches

**MCP represents the only viable path to cross-platform compatibility** and should be prioritized for shared functionality.

Teams must either:
1. **Standardize on one platform** (simplicity)
2. **Support multiple platforms** with isolated configs (flexibility)
3. **Invest in MCP** for shared logic (future-proof)

The research contained in `multi-platform-HARD-RULES.md` and `multi-platform-OPINIONS.md` provides comprehensive guidance for navigating this fragmented landscape.

---

## Next Steps

### Immediate Actions
1. ✅ Research complete
2. ✅ Documents generated
3. ⬜ Share with team
4. ⬜ Incorporate into project standards
5. ⬜ Update existing configurations

### Ongoing Maintenance
1. ⬜ Set quarterly review reminder
2. ⬜ Subscribe to platform release notes
3. ⬜ Monitor MCP specification updates
4. ⬜ Track community discussions

### Future Research
1. ⬜ Hands-on platform comparisons
2. ⬜ Performance benchmarking
3. ⬜ Enterprise feature analysis
4. ⬜ Emerging standards monitoring

---

**Research conducted by:** Claude Code (Anthropic)  
**Date:** 2026-01-31  
**Status:** Complete ✅
