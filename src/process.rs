use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

use crate::agents::{AGENTS, AgentEntry, CMDLINE_MARKERS};

pub(crate) fn find_agent_in_parent_tree() -> Option<String> {
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_cmd(UpdateKind::OnlyIfNotSet)
            .with_exe(UpdateKind::OnlyIfNotSet),
    );

    let current_pid = Pid::from_u32(std::process::id());
    let mut pid = system
        .process(current_pid)
        .and_then(sysinfo::Process::parent);

    while let Some(p) = pid {
        let proc = system.process(p)?;

        for agent in AGENTS {
            if process_matches(proc, agent) {
                if agent.name == "claude-code" && crate::is_cowork_override() {
                    return Some("cowork".to_string());
                }
                return Some(agent.name.to_string());
            }
        }

        pid = proc.parent();
    }

    None
}

pub(crate) fn is_process_match(name: &str, candidate: &str) -> bool {
    strip_suffix_ignore_ascii_case(name, ".exe").eq_ignore_ascii_case(candidate)
}

fn process_matches(proc: &Process, agent: &AgentEntry) -> bool {
    if let Some(name) = proc.name().to_str() {
        if agent
            .process_names
            .iter()
            .any(|c| is_process_match(name, c))
        {
            return true;
        }
    }

    // Interpreter-based CLIs (Python console scripts, Node/Bun wrappers) expose
    // the launcher as an argument rather than as the process name. Check the
    // executable and script arguments, but not arbitrary user arguments.
    proc.cmd()
        .iter()
        .take(2)
        .filter_map(|arg| arg.to_str())
        .any(|arg| {
            agent.process_names.iter().any(|c| is_cmdline_match(arg, c))
                || cmdline_marker_matches(arg, agent.name)
        })
}

fn is_cmdline_match(arg: &str, candidate: &str) -> bool {
    // Command lines may use either separator regardless of the current host
    // (for example Windows paths in a Git Bash or WSL environment).
    let name = arg.rsplit(['/', '\\']).next().unwrap_or(arg);
    strip_script_extension(name).eq_ignore_ascii_case(candidate)
}

fn strip_script_extension(name: &str) -> &str {
    for extension in [
        ".exe", ".cmd", ".bat", ".ps1", ".js", ".cjs", ".mjs", ".py", ".sh",
    ] {
        let stripped = strip_suffix_ignore_ascii_case(name, extension);
        if stripped.len() != name.len() {
            return stripped;
        }
    }
    name
}

fn cmdline_marker_matches(arg: &str, agent_name: &str) -> bool {
    let Some((_, markers)) = CMDLINE_MARKERS.iter().find(|(name, _)| *name == agent_name) else {
        return false;
    };
    let normalized = arg.replace('\\', "/");
    markers.iter().any(|marker| normalized.contains(marker))
}

fn strip_suffix_ignore_ascii_case<'a>(value: &'a str, suffix: &str) -> &'a str {
    let value_bytes = value.as_bytes();
    let suffix_bytes = suffix.as_bytes();
    if value_bytes.len() >= suffix_bytes.len()
        && value_bytes[value_bytes.len() - suffix_bytes.len()..].eq_ignore_ascii_case(suffix_bytes)
    {
        &value[..value.len() - suffix_bytes.len()]
    } else {
        value
    }
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
        assert!(is_process_match("Claude.EXE", "claude"));
    }

    #[test]
    fn no_match_different_name() {
        assert!(!is_process_match("cursor", "claude"));
    }

    #[test]
    fn exe_suffix_in_name() {
        assert!(!is_process_match("codex", "codex.exe"));
    }

    #[test]
    fn cmdline_script_path() {
        assert!(is_cmdline_match("/usr/bin/python3", "python3"));
        assert!(is_cmdline_match("/home/user/.local/bin/tau", "tau"));
    }

    #[test]
    fn cmdline_js_wrapper() {
        assert!(is_cmdline_match(
            "/usr/local/lib/node_modules/mimo.js",
            "mimo"
        ));
        assert!(is_cmdline_match("/opt/tools/mimo.exe", "mimo"));
        assert!(is_cmdline_match(
            r"C:\Users\me\AppData\Roaming\npm\mimo.cmd",
            "mimo"
        ));
        assert!(is_cmdline_match(r"C:\tools\MIMO.EXE", "mimo"));
    }

    #[test]
    fn cmdline_package_marker() {
        assert!(cmdline_marker_matches(
            r"C:\Users\me\.minimax-code\releases\1\lib\node_modules\@minimax-ai\code\cli.js",
            "minimax-code"
        ));
        assert!(cmdline_marker_matches(
            "/usr/local/lib/node_modules/@continuedev/cli/dist/cn.js",
            "continue"
        ));
        assert!(!cmdline_marker_matches("/tmp/notes.md", "minimax-code"));
    }

    #[test]
    fn cmdline_non_script_argument() {
        assert!(!is_cmdline_match("/usr/bin/python3", "tau"));
        assert!(!is_cmdline_match("/tmp/notes.md", "tau"));
        assert!(!is_cmdline_match(r"C:\tools\notes.md", "tau"));
    }
}
