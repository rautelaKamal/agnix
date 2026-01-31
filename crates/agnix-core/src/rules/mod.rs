//! Validation rules

pub mod skill;
pub mod agent;
pub mod hooks;
pub mod plugin;
pub mod claude_md;
pub mod xml;
pub mod imports;

use crate::{
    config::LintConfig,
    diagnostics::Diagnostic,
};
use std::path::Path;

/// Trait for file validators
pub trait Validator {
    fn validate(&self, path: &Path, content: &str, config: &LintConfig) -> Vec<Diagnostic>;
}
