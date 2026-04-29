use std::env;

use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::agents::AGENTS;

pub fn find_agent_in_parent_tree() -> Option<String> {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);

    let current_pid = Pid::from_u32(std::process::id());
    let mut pid = Some(current_pid);

    while let Some(p) = pid {
        let proc = system.process(p)?;

        let Some(name_raw) = proc.name().to_str() else {
            pid = proc.parent();
            continue;
        };

        for agent in AGENTS {
            for &candidate in agent.process_names {
                if is_process_match(name_raw, candidate) {
                    if agent.name == "claude-code" && env::var("CLAUDE_CODE_IS_COWORK").is_ok() {
                        return Some("cowork".to_string());
                    }
                    return Some(agent.name.to_string());
                }
            }
        }

        pid = proc.parent();
    }

    None
}

pub(crate) fn is_process_match(name: &str, candidate: &str) -> bool {
    name.strip_suffix(".exe")
        .unwrap_or(name)
        .eq_ignore_ascii_case(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match() {
        assert!(is_process_match("claude", "claude"));
    }

    #[test]
    fn case_insensitive() {
        assert!(is_process_match("Claude", "claude"));
        assert!(is_process_match("CLAUDE", "claude"));
    }

    #[test]
    fn strip_exe_suffix() {
        assert!(is_process_match("claude.exe", "claude"));
    }

    #[test]
    fn no_match_different_name() {
        assert!(!is_process_match("cursor", "claude"));
    }

    #[test]
    fn exe_suffix_in_name() {
        assert!(!is_process_match("codex", "codex.exe"));
    }
}
