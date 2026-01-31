//! Skill file validation

use crate::{
    config::LintConfig,
    diagnostics::Diagnostic,
    parsers::parse_frontmatter,
    rules::Validator,
    schemas::SkillSchema,
};
use std::path::{Path, PathBuf};

pub struct SkillValidator;

impl Validator for SkillValidator {
    fn validate(&self, path: &Path, content: &str, config: &LintConfig) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        if !config.rules.frontmatter_validation {
            return diagnostics;
        }

        // Parse frontmatter
        let result: Result<(SkillSchema, String), _> = parse_frontmatter(content);

        match result {
            Ok((schema, _body)) => {
                // Run schema validations
                let errors = schema.validate();
                for error in errors {
                    diagnostics.push(Diagnostic::error(
                        path.to_path_buf(),
                        1, // TODO: Get actual line from error
                        0,
                        "skill::schema",
                        error,
                    ));
                }
            }
            Err(e) => {
                diagnostics.push(Diagnostic::error(
                    path.to_path_buf(),
                    1,
                    0,
                    "skill::parse",
                    format!("Failed to parse SKILL.md: {}", e),
                ));
            }
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LintConfig;

    #[test]
    fn test_valid_skill() {
        let content = r#"---
name: test-skill
description: A test skill for validation
---
Skill body content"#;

        let validator = SkillValidator;
        let diagnostics = validator.validate(
            Path::new("test.md"),
            content,
            &LintConfig::default(),
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_invalid_skill_name() {
        let content = r#"---
name: Test-Skill
description: A test skill
---
Body"#;

        let validator = SkillValidator;
        let diagnostics = validator.validate(
            Path::new("test.md"),
            content,
            &LintConfig::default(),
        );

        assert!(!diagnostics.is_empty());
    }
}
