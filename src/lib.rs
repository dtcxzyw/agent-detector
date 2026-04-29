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
    /// The agent name (e.g. `"opencode"`, `"claude-code"`), always lowercase.
    /// For unknown `AI_AGENT` / `AGENT` values, this is the lowercased literal value.
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

/// Returns the detected agent name (always lowercase), or `None` if no agent is detected.
#[must_use]
pub fn agent_name() -> Option<String> {
    detect().map(|info| info.name)
}

fn check_standard_env_vars() -> Option<AgentInfo> {
    const STANDARD_VARS: &[&str] = &["AI_AGENT", "AGENT"];

    for &var in STANDARD_VARS {
        if let Ok(value) = env::var(var) {
            let value = value.trim().to_ascii_lowercase();
            if !value.is_empty() {
                return Some(AgentInfo {
                    name: value,
                    source: DetectionSource::StandardEnvVar,
                });
            }
        }
    }

    None
}

fn check_tool_env_vars() -> Option<AgentInfo> {
    for agent in agents::AGENTS {
        for &env_var in agent.env_vars {
            if env_var_is_set(env_var) {
                if agent.name == "claude-code" && env::var("CLAUDE_CODE_IS_COWORK").is_ok() {
                    return Some(AgentInfo {
                        name: "cowork".to_string(),
                        source: DetectionSource::ToolEnvVar,
                    });
                }
                return Some(AgentInfo {
                    name: agent.name.to_string(),
                    source: DetectionSource::ToolEnvVar,
                });
            }
        }
    }

    None
}

fn env_var_is_set(var: &str) -> bool {
    if var == "CURSOR_EXTENSION_HOST_ROLE" {
        env::var(var).is_ok_and(|v| v == "agent-exec")
    } else {
        env::var(var).is_ok()
    }
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
    fn test_detect_with_env_var() {
        unsafe { set_env("OPENCODE_CLIENT", "1") };
        let result = detect();
        unsafe { remove_env("OPENCODE_CLIENT") };

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "opencode");
    }

    #[test]
    fn test_is_agent_with_env_var() {
        unsafe { set_env("CLAUDECODE", "1") };
        assert!(is_agent());
        unsafe { remove_env("CLAUDECODE") };
    }

    #[test]
    fn test_agent_name_with_env_var() {
        unsafe { set_env("AI_AGENT", "Test-Bot") };
        let result = check_standard_env_vars();
        unsafe { remove_env("AI_AGENT") };

        assert_eq!(result.unwrap().name, "test-bot");
    }

    #[test]
    fn test_check_standard_env_var_ai_agent() {
        unsafe { set_env("AI_AGENT", "OpenCode") };
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
    fn test_cursor_extension_host_role_agent_exec() {
        unsafe { set_env("CURSOR_EXTENSION_HOST_ROLE", "agent-exec") };
        let result = check_tool_env_vars();
        unsafe { remove_env("CURSOR_EXTENSION_HOST_ROLE") };

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "cursor-cli");
    }

    #[test]
    fn test_cursor_extension_host_role_other_value() {
        unsafe { set_env("CURSOR_EXTENSION_HOST_ROLE", "editor") };
        let result = check_tool_env_vars();
        unsafe { remove_env("CURSOR_EXTENSION_HOST_ROLE") };

        assert!(result.is_none());
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
}
