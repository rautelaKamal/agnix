//! Hooks validation (stub)

use crate::{config::LintConfig, diagnostics::Diagnostic, rules::Validator};
use std::path::Path;

pub struct HooksValidator;

impl Validator for HooksValidator {
    fn validate(&self, _path: &Path, _content: &str, _config: &LintConfig) -> Vec<Diagnostic> {
        // TODO: Implement hooks validation
        Vec::new()
    }
}
