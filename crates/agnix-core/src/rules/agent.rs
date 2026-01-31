//! Agent file validation (stub)

use crate::{config::LintConfig, diagnostics::Diagnostic, rules::Validator};
use std::path::Path;

pub struct AgentValidator;

impl Validator for AgentValidator {
    fn validate(&self, _path: &Path, _content: &str, _config: &LintConfig) -> Vec<Diagnostic> {
        // TODO: Implement agent validation
        Vec::new()
    }
}
