//! CLAUDE.md validation

use crate::{
    config::LintConfig,
    diagnostics::Diagnostic,
    rules::Validator,
    schemas::claude_md::find_generic_instructions,
};
use std::path::Path;

pub struct ClaudeMdValidator;

impl Validator for ClaudeMdValidator {
    fn validate(&self, path: &Path, content: &str, config: &LintConfig) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        if !config.rules.generic_instructions {
            return diagnostics;
        }

        // Check for generic instructions
        let generic_insts = find_generic_instructions(content);
        for inst in generic_insts {
            diagnostics.push(
                Diagnostic::warning(
                    path.to_path_buf(),
                    inst.line,
                    inst.column,
                    "claude_md::generic",
                    format!("Generic instruction '{}' - Claude already knows this", inst.text),
                )
                .with_suggestion(
                    "Remove generic instructions. Focus on project-specific context.".to_string(),
                ),
            );
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LintConfig;

    #[test]
    fn test_generic_instruction_detected() {
        let content = "Be helpful and accurate when responding.";
        let validator = ClaudeMdValidator;
        let diagnostics = validator.validate(
            Path::new("CLAUDE.md"),
            content,
            &LintConfig::default(),
        );

        assert!(!diagnostics.is_empty());
    }
}
