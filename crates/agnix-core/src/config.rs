//! Linter configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the linter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintConfig {
    /// Severity level threshold
    pub severity: SeverityLevel,

    /// Rules to enable/disable
    pub rules: RuleConfig,

    /// Paths to exclude
    pub exclude: Vec<String>,

    /// Target tool (claude-code, cursor, codex, generic)
    pub target: TargetTool,
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            severity: SeverityLevel::Warning,
            rules: RuleConfig::default(),
            exclude: vec![
                "node_modules/**".to_string(),
                ".git/**".to_string(),
                "target/**".to_string(),
            ],
            target: TargetTool::Generic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SeverityLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Detect generic instructions in CLAUDE.md
    pub generic_instructions: bool,

    /// Validate YAML frontmatter
    pub frontmatter_validation: bool,

    /// Check XML tag balance
    pub xml_balance: bool,

    /// Validate @import references
    pub import_references: bool,

    /// Validate tool names
    pub tool_names: bool,

    /// Check required fields
    pub required_fields: bool,
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            generic_instructions: true,
            frontmatter_validation: true,
            xml_balance: true,
            import_references: true,
            tool_names: true,
            required_fields: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetTool {
    /// Generic Agent Skills standard
    Generic,
    /// Claude Code specific
    ClaudeCode,
    /// Cursor specific
    Cursor,
    /// Codex specific
    Codex,
}

impl LintConfig {
    /// Load config from file
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load config or use default
    pub fn load_or_default(path: Option<&PathBuf>) -> Self {
        path.and_then(|p| Self::load(p).ok())
            .unwrap_or_default()
    }
}
