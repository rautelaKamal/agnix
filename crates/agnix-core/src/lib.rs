//! # agnix-core
//!
//! Core validation engine for agent configurations.
//!
//! Validates:
//! - Agent Skills (SKILL.md)
//! - Agent definitions (.md files with frontmatter)
//! - MCP tool configurations
//! - Claude Code hooks
//! - CLAUDE.md memory files
//! - Plugin manifests

pub mod config;
pub mod diagnostics;
pub mod parsers;
pub mod rules;
pub mod schemas;

pub use config::LintConfig;
pub use diagnostics::{Diagnostic, DiagnosticLevel, LintError, LintResult};

/// Main entry point for validating a project
pub fn validate_project(path: &std::path::Path, config: &LintConfig) -> LintResult<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    // TODO: Walk directory tree
    // TODO: Detect file types
    // TODO: Apply appropriate validators
    // TODO: Collect diagnostics

    Ok(diagnostics)
}

/// Validate a single file
pub fn validate_file(path: &std::path::Path, config: &LintConfig) -> LintResult<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    // TODO: Detect file type
    // TODO: Parse file
    // TODO: Run appropriate validators
    // TODO: Collect diagnostics

    Ok(diagnostics)
}
