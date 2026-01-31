//! JSON parser for hooks and plugin configs

use crate::diagnostics::{LintError, LintResult};
use serde::de::DeserializeOwned;

/// Parse JSON config file
pub fn parse_json_config<T: DeserializeOwned>(content: &str) -> LintResult<T> {
    let parsed: T = serde_json::from_str(content).map_err(|e| LintError::Other(e.into()))?;
    Ok(parsed)
}
