//! Markdown parser for extracting @imports and checking XML tags

use regex::Regex;
use std::sync::OnceLock;

static IMPORT_REGEX: OnceLock<Regex> = OnceLock::new();
static XML_TAG_REGEX: OnceLock<Regex> = OnceLock::new();

/// Extract @import references from markdown content
pub fn extract_imports(content: &str) -> Vec<Import> {
    let re = IMPORT_REGEX.get_or_init(|| {
        Regex::new(r"@([^\s\]]+)").unwrap()
    });

    let mut imports = Vec::new();
    for cap in re.captures_iter(content) {
        if let Some(path_match) = cap.get(1) {
            let path = path_match.as_str().to_string();
            let start = path_match.start();
            imports.push(Import {
                path,
                line: content[..start].lines().count(),
                column: start - content[..start].rfind('\n').unwrap_or(0),
            });
        }
    }

    imports
}

/// Extract XML tags for balance checking
pub fn extract_xml_tags(content: &str) -> Vec<XmlTag> {
    let re = XML_TAG_REGEX.get_or_init(|| {
        Regex::new(r"<(/?)([a-zA-Z_][a-zA-Z0-9_-]*)>").unwrap()
    });

    let mut tags = Vec::new();
    for cap in re.captures_iter(content) {
        let is_closing = cap.get(1).map_or(false, |m| m.as_str() == "/");
        if let Some(name_match) = cap.get(2) {
            let name = name_match.as_str().to_string();
            let start = cap.get(0).unwrap().start();
            tags.push(XmlTag {
                name,
                is_closing,
                line: content[..start].lines().count(),
                column: start - content[..start].rfind('\n').unwrap_or(0),
            });
        }
    }

    tags
}

/// Check if XML tags are balanced
pub fn check_xml_balance(tags: &[XmlTag]) -> Vec<XmlBalanceError> {
    let mut stack: Vec<&XmlTag> = Vec::new();
    let mut errors = Vec::new();

    for tag in tags {
        if tag.is_closing {
            if let Some(last) = stack.last() {
                if last.name == tag.name {
                    stack.pop();
                } else {
                    errors.push(XmlBalanceError::Mismatch {
                        expected: last.name.clone(),
                        found: tag.name.clone(),
                        line: tag.line,
                        column: tag.column,
                    });
                }
            } else {
                errors.push(XmlBalanceError::UnmatchedClosing {
                    tag: tag.name.clone(),
                    line: tag.line,
                    column: tag.column,
                });
            }
        } else {
            stack.push(tag);
        }
    }

    // Unclosed tags
    for tag in stack {
        errors.push(XmlBalanceError::Unclosed {
            tag: tag.name.clone(),
            line: tag.line,
            column: tag.column,
        });
    }

    errors
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct XmlTag {
    pub name: String,
    pub is_closing: bool,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum XmlBalanceError {
    Unclosed { tag: String, line: usize, column: usize },
    UnmatchedClosing { tag: String, line: usize, column: usize },
    Mismatch { expected: String, found: String, line: usize, column: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_imports() {
        let content = "See @docs/guide.md and @README.md";
        let imports = extract_imports(content);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].path, "docs/guide.md");
        assert_eq!(imports[1].path, "README.md");
    }

    #[test]
    fn test_xml_balance() {
        let content = "<example>test</example>";
        let tags = extract_xml_tags(content);
        let errors = check_xml_balance(&tags);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_xml_unclosed() {
        let content = "<example>test";
        let tags = extract_xml_tags(content);
        let errors = check_xml_balance(&tags);
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], XmlBalanceError::Unclosed { .. }));
    }
}
