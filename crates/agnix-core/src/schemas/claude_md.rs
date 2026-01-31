//! CLAUDE.md validation rules

use regex::Regex;
use std::sync::OnceLock;

static GENERIC_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

/// Generic instruction patterns that Claude already knows
pub fn generic_patterns() -> &'static Vec<Regex> {
    GENERIC_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"(?i)\bbe\s+helpful").unwrap(),
            Regex::new(r"(?i)\bbe\s+accurate").unwrap(),
            Regex::new(r"(?i)\bthink\s+step\s+by\s+step").unwrap(),
            Regex::new(r"(?i)\bbe\s+concise").unwrap(),
            Regex::new(r"(?i)\bformat.*properly").unwrap(),
            Regex::new(r"(?i)\bprovide.*clear.*explanations").unwrap(),
            Regex::new(r"(?i)\bmake\s+sure\s+to").unwrap(),
            Regex::new(r"(?i)\balways\s+be").unwrap(),
        ]
    })
}

/// Check for generic instructions in content
pub fn find_generic_instructions(content: &str) -> Vec<GenericInstruction> {
    let mut results = Vec::new();
    let patterns = generic_patterns();

    for (line_num, line) in content.lines().enumerate() {
        for pattern in patterns {
            if let Some(mat) = pattern.find(line) {
                results.push(GenericInstruction {
                    line: line_num + 1,
                    column: mat.start(),
                    text: mat.as_str().to_string(),
                    pattern: pattern.as_str().to_string(),
                });
            }
        }
    }

    results
}

#[derive(Debug, Clone)]
pub struct GenericInstruction {
    pub line: usize,
    pub column: usize,
    pub text: String,
    pub pattern: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_generic_instructions() {
        let content = "Be helpful and accurate when responding.\nUse project-specific guidelines.";
        let results = find_generic_instructions(content);
        assert!(!results.is_empty());
        assert!(results[0].text.to_lowercase().contains("helpful"));
    }

    #[test]
    fn test_no_generic_instructions() {
        let content = "Use the coding style defined in .editorconfig\nFollow team conventions";
        let results = find_generic_instructions(content);
        assert!(results.is_empty());
    }
}
