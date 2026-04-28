//! Detects whether the current process is being invoked by an AI coding agent.
//!
//! Detection is performed in three tiers:
//! 1. Parent process tree matching (requires `process-tree` feature)
//! 2. `AI_AGENT` / `AGENT` standard environment variables
//! 3. Tool-specific environment variables
//!
//! # Example
//!
//! ```rust
//! use agent_detector::is_agent;
//!
//! if is_agent() {
//!     eprintln!("Running under an AI agent");
//! }
//! ```

use std::env;

mod agents;
#[cfg(feature = "process-tree")]
mod process;

/// Information about the detected agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentInfo {
    /// The agent name (e.g. `"opencode"`, `"claude-code"`).
    /// For unknown `AI_AGENT` / `AGENT` values, this is the literal value.
    pub name: String,
    /// How the agent was detected.
    pub source: DetectionSource,
}

/// Indicates which detection tier produced the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionSource {
    /// Detected via parent process tree walking.
    #[cfg(feature = "process-tree")]
    ParentProcess,
    /// Detected via `AI_AGENT` or `AGENT` standard environment variable.
    StandardEnvVar,
    /// Detected via a tool-specific environment variable.
    ToolEnvVar,
}

/// Run full agent detection (L1 → L2 → L3).
///
/// Returns `None` if no agent is detected.
#[must_use]
pub fn detect() -> Option<AgentInfo> {
    #[cfg(feature = "process-tree")]
    if let Some(name) = process::find_agent_in_parent_tree() {
        return Some(AgentInfo {
            name,
            source: DetectionSource::ParentProcess,
        });
    }

    if let Some(info) = check_standard_env_vars() {
        return Some(info);
    }

    if let Some(info) = check_tool_env_vars() {
        return Some(info);
    }

    None
}

/// Returns `true` if the current process is running under an AI agent.
#[must_use]
pub fn is_agent() -> bool {
    detect().is_some()
}

/// Returns the detected agent name, or `None` if no agent is detected.
#[must_use]
pub fn agent_name() -> Option<String> {
    detect().map(|info| info.name)
}

fn check_standard_env_vars() -> Option<AgentInfo> {
    const STANDARD_VARS: &[&str] = &["AI_AGENT", "AGENT"];

    for &var in STANDARD_VARS {
        if let Ok(value) = env::var(var) {
            let value = value.trim();
            if !value.is_empty() {
                return Some(AgentInfo {
                    name: value.to_string(),
                    source: DetectionSource::StandardEnvVar,
                });
            }
        }
    }

    None
}

fn check_tool_env_vars() -> Option<AgentInfo> {
    let cowork = check_cowork();
    if let Some(info) = cowork {
        return Some(info);
    }

    for tool_agent in agents::TOOL_AGENTS {
        if tool_agent.name == "cowork" {
            continue;
        }

        for &env_var in tool_agent.env_vars {
            if env::var(env_var).is_ok() {
                return Some(AgentInfo {
                    name: tool_agent.name.to_string(),
                    source: DetectionSource::ToolEnvVar,
                });
            }
        }
    }

    None
}

fn check_cowork() -> Option<AgentInfo> {
    if env::var("CLAUDE_CODE_IS_COWORK").is_ok()
        && (env::var("CLAUDECODE").is_ok() || env::var("CLAUDE_CODE").is_ok())
    {
        return Some(AgentInfo {
            name: "cowork".to_string(),
            source: DetectionSource::ToolEnvVar,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn set_env(key: &str, value: &str) {
        unsafe { env::set_var(key, value) };
    }

    unsafe fn remove_env(key: &str) {
        unsafe { env::remove_var(key) };
    }

    #[test]
    #[cfg(feature = "process-tree")]
    fn test_detect_in_parent_process() {
        let result = detect();
        assert!(result.is_some());
        assert_eq!(result.unwrap().source, DetectionSource::ParentProcess);
    }

    #[test]
    #[cfg(feature = "process-tree")]
    fn test_is_agent_true() {
        assert!(is_agent());
    }

    #[test]
    fn test_check_standard_env_var_ai_agent() {
        unsafe { set_env("AI_AGENT", "opencode") };
        let result = check_standard_env_vars();
        unsafe { remove_env("AI_AGENT") };

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "opencode");
        assert_eq!(info.source, DetectionSource::StandardEnvVar);
    }

    #[test]
    fn test_check_standard_env_var_agent() {
        unsafe { set_env("AGENT", "my-custom-agent") };
        let result = check_standard_env_vars();
        unsafe { remove_env("AGENT") };

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "my-custom-agent");
        assert_eq!(info.source, DetectionSource::StandardEnvVar);
    }

    #[test]
    fn test_check_standard_env_var_ai_agent_priority() {
        unsafe { set_env("AI_AGENT", "first") };
        unsafe { set_env("AGENT", "second") };
        let result = check_standard_env_vars();
        unsafe { remove_env("AI_AGENT") };
        unsafe { remove_env("AGENT") };

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "first");
    }

    #[test]
    fn test_check_standard_env_var_none() {
        let result = check_standard_env_vars();
        assert!(result.is_none());
    }

    #[test]
    fn test_check_tool_env_var_cursor() {
        unsafe { set_env("CURSOR_TRACE_ID", "abc123") };
        let result = check_tool_env_vars();
        unsafe { remove_env("CURSOR_TRACE_ID") };

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "cursor");
        assert_eq!(info.source, DetectionSource::ToolEnvVar);
    }

    #[test]
    fn test_check_tool_env_var_claude() {
        unsafe { set_env("CLAUDECODE", "1") };
        let result = check_tool_env_vars();
        unsafe { remove_env("CLAUDECODE") };

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "claude-code");
    }

    #[test]
    fn test_cowork_requires_claudecode() {
        unsafe { set_env("CLAUDE_CODE_IS_COWORK", "1") };
        let result = check_tool_env_vars();
        unsafe { remove_env("CLAUDE_CODE_IS_COWORK") };

        assert!(result.is_none());
    }

    #[test]
    fn test_cowork_with_claudecode() {
        unsafe { set_env("CLAUDE_CODE_IS_COWORK", "1") };
        unsafe { set_env("CLAUDECODE", "1") };
        let result = check_tool_env_vars();
        unsafe { remove_env("CLAUDE_CODE_IS_COWORK") };
        unsafe { remove_env("CLAUDECODE") };

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "cowork");
    }

    #[test]
    fn test_check_tool_env_var_gemini() {
        unsafe { set_env("GEMINI_CLI", "1") };
        let result = check_tool_env_vars();
        unsafe { remove_env("GEMINI_CLI") };

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "gemini");
    }

    #[test]
    fn test_check_tool_env_var_codex() {
        unsafe { set_env("CODEX_CI", "1") };
        let result = check_tool_env_vars();
        unsafe { remove_env("CODEX_CI") };

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "codex");
    }

    #[test]
    fn test_check_tool_env_var_opencode() {
        unsafe { set_env("OPENCODE_CLIENT", "1") };
        let result = check_tool_env_vars();
        unsafe { remove_env("OPENCODE_CLIENT") };

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "opencode");
    }

    #[test]
    fn test_check_tool_env_var_none() {
        let result = check_tool_env_vars();
        assert!(result.is_none());
    }

    #[test]
    fn test_agent_name() {
        let name = agent_name();
        assert!(name.is_some());
    }
}
