//! Schema definitions for agent config files

pub mod skill;
pub mod agent;
pub mod hooks;
pub mod plugin;
pub mod claude_md;

pub use skill::SkillSchema;
pub use agent::AgentSchema;
pub use hooks::HooksSchema;
pub use plugin::PluginSchema;
