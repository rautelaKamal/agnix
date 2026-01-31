//! Hooks schema (Claude Code hooks)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Hooks configuration schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksSchema {
    pub hooks: HashMap<String, Vec<HookMatcher>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookMatcher {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
    pub hooks: Vec<Hook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Hook {
    #[serde(rename = "command")]
    Command {
        command: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout: Option<u64>,
    },
    #[serde(rename = "prompt")]
    Prompt {
        prompt: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout: Option<u64>,
    },
}

impl HooksSchema {
    /// Valid hook events
    const VALID_EVENTS: &'static [&'static str] = &[
        "PreToolUse",
        "PermissionRequest",
        "PostToolUse",
        "PostToolUseFailure",
        "Notification",
        "UserPromptSubmit",
        "Stop",
        "SubagentStart",
        "SubagentStop",
        "PreCompact",
        "Setup",
        "SessionStart",
        "SessionEnd",
    ];

    /// Validate hook events
    pub fn validate_events(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for event in self.hooks.keys() {
            if !Self::VALID_EVENTS.contains(&event.as_str()) {
                errors.push(format!("Unknown hook event '{}', valid events: {:?}", event, Self::VALID_EVENTS));
            }
        }

        errors
    }

    /// Validate hook structure
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        errors.extend(self.validate_events());

        // Validate each hook has required fields
        for (event, matchers) in &self.hooks {
            for (i, matcher) in matchers.iter().enumerate() {
                if matcher.hooks.is_empty() {
                    errors.push(format!("Hook event '{}' matcher {} has empty hooks array", event, i));
                }
            }
        }

        errors
    }
}
