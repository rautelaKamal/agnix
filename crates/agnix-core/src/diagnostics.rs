//! Diagnostic types and error reporting

use miette::{Diagnostic as MietteDiagnostic, SourceSpan};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

pub type LintResult<T> = Result<T, LintError>;

/// A diagnostic message from the linter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub rule: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

impl Diagnostic {
    pub fn error(file: PathBuf, line: usize, column: usize, rule: &str, message: String) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message,
            file,
            line,
            column,
            rule: rule.to_string(),
            suggestion: None,
        }
    }

    pub fn warning(file: PathBuf, line: usize, column: usize, rule: &str, message: String) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message,
            file,
            line,
            column,
            rule: rule.to_string(),
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

/// Linter errors
#[derive(Error, Debug, MietteDiagnostic)]
pub enum LintError {
    #[error("Failed to read file: {path}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse YAML frontmatter")]
    #[diagnostic(
        code(agnix::yaml_parse),
        help("Check YAML syntax between --- markers")
    )]
    YamlParse {
        #[source_code]
        src: String,
        #[label("Parse error here")]
        span: SourceSpan,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("Failed to parse JSON")]
    #[diagnostic(code(agnix::json_parse), help("Check JSON syntax"))]
    JsonParse {
        #[source_code]
        src: String,
        #[label("Parse error here")]
        span: SourceSpan,
        #[source]
        source: serde_json::Error,
    },

    #[error("Generic instruction detected")]
    #[diagnostic(
        code(agnix::generic_instruction),
        help("Remove generic instructions. Claude already knows to be helpful.")
    )]
    GenericInstruction {
        #[source_code]
        src: String,
        #[label("This instruction is redundant")]
        span: SourceSpan,
    },

    #[error("Invalid skill name: {name}")]
    #[diagnostic(
        code(agnix::invalid_name),
        help("Use lowercase letters and hyphens only (e.g., 'code-review')")
    )]
    InvalidName {
        name: String,
        #[source_code]
        src: String,
        #[label("Must be lowercase with hyphens")]
        span: SourceSpan,
    },

    #[error("Missing required field: {field}")]
    #[diagnostic(code(agnix::missing_field))]
    MissingField {
        field: String,
        #[source_code]
        src: String,
        #[label("Add '{field}' field here")]
        span: SourceSpan,
    },

    #[error("XML tag mismatch")]
    #[diagnostic(
        code(agnix::xml_mismatch),
        help("Ensure all XML tags are properly closed")
    )]
    XmlMismatch {
        #[source_code]
        src: String,
        #[label("Unclosed or mismatched tag")]
        span: SourceSpan,
        open_tag: String,
    },

    #[error("Import not found: {path}")]
    #[diagnostic(code(agnix::import_not_found), help("Check the file path exists"))]
    ImportNotFound {
        path: String,
        #[source_code]
        src: String,
        #[label("File not found")]
        span: SourceSpan,
    },

    #[error("Unknown tool: {tool}")]
    #[diagnostic(
        code(agnix::unknown_tool),
        help("Check valid tool names for your target")
    )]
    UnknownTool {
        tool: String,
        #[source_code]
        src: String,
        #[label("Unknown tool")]
        span: SourceSpan,
    },

    #[error(transparent)]
    Other(anyhow::Error),
}
