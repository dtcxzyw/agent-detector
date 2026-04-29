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
#[non_exhaustive]
pub struct AgentInfo {
    /// The agent name (e.g. `"opencode"`, `"claude-code"`), always lowercase.
    /// For unknown `AI_AGENT` / `AGENT` values, this is the lowercased literal value.
    pub name: String,
    /// How the agent was detected.
    pub source: DetectionSource,
}

/// Indicates which detection tier produced the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
///
/// No tests cover the L1 > L2 > L3 priority chain. Priority is structurally
/// guaranteed by the sequential `if let Some(…) { return … }` pattern, and
/// each tier is tested independently. Adding combinatorial priority tests
/// would increase maintenance cost without proportional benefit.
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

fn is_cowork_override() -> bool {
    env::var("CLAUDE_CODE_IS_COWORK").is_ok_and(|v| !v.is_empty())
}

fn check_tool_env_vars() -> Option<AgentInfo> {
    for agent in agents::AGENTS {
        for &env_var in agent.env_vars {
            if env_var_is_set(env_var) {
                if agent.name == "claude-code" && is_cowork_override() {
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
        env::var(var).is_ok_and(|v| !v.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EnvGuard {
        key: &'static str,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            // SAFETY: tests run single-threaded (RUST_TEST_THREADS=1), and
            // EnvGuard cleans up on Drop, so no race conditions exist.
            unsafe { std::env::set_var(key, value) };
            EnvGuard { key }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            // SAFETY: cleanup paired with the set() in the same guard;
            // still single-threaded.
            unsafe { std::env::remove_var(self.key) };
        }
    }

    #[test]
    fn test_detect_with_env_var() {
        let _guard = EnvGuard::set("OPENCODE_CLIENT", "1");
        let result = detect();

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "opencode");
    }

    #[test]
    fn test_is_agent_with_env_var() {
        let _guard = EnvGuard::set("CLAUDECODE", "1");
        assert!(is_agent());
    }

    #[test]
    fn test_agent_name_with_env_var() {
        let _guard = EnvGuard::set("AI_AGENT", "Test-Bot");
        let result = check_standard_env_vars();

        assert_eq!(result.unwrap().name, "test-bot");
    }

    #[test]
    fn test_check_standard_env_var_ai_agent() {
        let _guard = EnvGuard::set("AI_AGENT", "OpenCode");
        let result = check_standard_env_vars();

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "opencode");
        assert_eq!(info.source, DetectionSource::StandardEnvVar);
    }

    #[test]
    fn test_check_standard_env_var_agent() {
        let _guard = EnvGuard::set("AGENT", "my-custom-agent");
        let result = check_standard_env_vars();

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "my-custom-agent");
        assert_eq!(info.source, DetectionSource::StandardEnvVar);
    }

    #[test]
    fn test_check_standard_env_var_ai_agent_priority() {
        let _ai = EnvGuard::set("AI_AGENT", "first");
        let _agent = EnvGuard::set("AGENT", "second");
        let result = check_standard_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "first");
    }

    #[test]
    fn test_check_standard_env_var_none() {
        let result = check_standard_env_vars();
        assert!(result.is_none());
    }

    #[test]
    fn test_check_standard_env_var_ai_agent_empty() {
        let _guard = EnvGuard::set("AI_AGENT", "");
        let result = check_standard_env_vars();
        assert!(result.is_none());
    }

    #[test]
    fn test_check_standard_env_var_ai_agent_whitespace() {
        let _guard = EnvGuard::set("AI_AGENT", "   ");
        let result = check_standard_env_vars();
        assert!(result.is_none());
    }

    #[test]
    fn test_check_standard_env_var_agent_empty() {
        let _guard = EnvGuard::set("AGENT", "");
        let result = check_standard_env_vars();
        assert!(result.is_none());
    }

    #[test]
    fn test_check_standard_env_var_agent_whitespace() {
        let _guard = EnvGuard::set("AGENT", "   ");
        let result = check_standard_env_vars();
        assert!(result.is_none());
    }

    #[test]
    fn test_check_tool_env_var_cursor() {
        let _guard = EnvGuard::set("CURSOR_TRACE_ID", "abc123");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.name, "cursor");
        assert_eq!(info.source, DetectionSource::ToolEnvVar);
    }

    #[test]
    fn test_cursor_extension_host_role_agent_exec() {
        let _guard = EnvGuard::set("CURSOR_EXTENSION_HOST_ROLE", "agent-exec");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "cursor-cli");
    }

    #[test]
    fn test_cursor_extension_host_role_other_value() {
        let _guard = EnvGuard::set("CURSOR_EXTENSION_HOST_ROLE", "editor");
        let result = check_tool_env_vars();

        assert!(result.is_none());
    }

    #[test]
    fn test_check_tool_env_var_claude() {
        let _guard = EnvGuard::set("CLAUDECODE", "1");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "claude-code");
    }

    #[test]
    fn test_cowork_requires_claudecode() {
        let _guard = EnvGuard::set("CLAUDE_CODE_IS_COWORK", "1");
        let result = check_tool_env_vars();

        assert!(result.is_none());
    }

    #[test]
    fn test_cowork_empty_cowork_value() {
        let _cw = EnvGuard::set("CLAUDE_CODE_IS_COWORK", "");
        let _claude = EnvGuard::set("CLAUDECODE", "1");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "claude-code");
    }

    #[test]
    fn test_cowork_with_claudecode() {
        let _cw = EnvGuard::set("CLAUDE_CODE_IS_COWORK", "1");
        let _claude = EnvGuard::set("CLAUDECODE", "1");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "cowork");
    }

    #[test]
    fn test_check_tool_env_var_gemini() {
        let _guard = EnvGuard::set("GEMINI_CLI", "1");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "gemini");
    }

    #[test]
    fn test_check_tool_env_var_codex() {
        let _guard = EnvGuard::set("CODEX_CI", "1");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "codex");
    }

    #[test]
    fn test_check_tool_env_var_opencode() {
        let _guard = EnvGuard::set("OPENCODE_CLIENT", "1");
        let result = check_tool_env_vars();

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "opencode");
    }

    #[test]
    fn test_check_tool_env_var_empty_value() {
        let _guard = EnvGuard::set("CLAUDECODE", "");
        let result = check_tool_env_vars();

        assert!(result.is_none());
    }

    #[test]
    fn test_check_tool_env_var_none() {
        let result = check_tool_env_vars();
        assert!(result.is_none());
    }
}
