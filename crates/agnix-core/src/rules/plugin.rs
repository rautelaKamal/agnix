//! Plugin manifest validation (stub)

use crate::{config::LintConfig, diagnostics::Diagnostic, rules::Validator};
use std::path::Path;

pub struct PluginValidator;

impl Validator for PluginValidator {
    fn validate(&self, _path: &Path, _content: &str, _config: &LintConfig) -> Vec<Diagnostic> {
        // TODO: Implement plugin validation
        Vec::new()
    }
}
