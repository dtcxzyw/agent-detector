#[derive(Debug)]
pub(crate) struct AgentEntry {
    pub name: &'static str,
    #[cfg_attr(not(feature = "process-tree"), allow(dead_code))]
    pub process_names: &'static [&'static str],
    pub env_vars: &'static [&'static str],
}

pub(crate) static AGENTS: &[AgentEntry] = &[
    AgentEntry {
        name: "cursor",
        process_names: &["cursor"],
        env_vars: &["CURSOR_TRACE_ID"],
    },
    AgentEntry {
        name: "cursor-cli",
        process_names: &[],
        env_vars: &["CURSOR_AGENT", "CURSOR_EXTENSION_HOST_ROLE"],
    },
    AgentEntry {
        name: "gemini",
        process_names: &["gemini"],
        env_vars: &["GEMINI_CLI"],
    },
    AgentEntry {
        name: "codex",
        process_names: &["codex"],
        env_vars: &["CODEX_SANDBOX", "CODEX_CI", "CODEX_THREAD_ID"],
    },
    AgentEntry {
        name: "antigravity",
        process_names: &["amp"],
        env_vars: &["ANTIGRAVITY_AGENT"],
    },
    AgentEntry {
        name: "augment-cli",
        process_names: &["augment-cli"],
        env_vars: &["AUGMENT_AGENT"],
    },
    AgentEntry {
        name: "opencode",
        process_names: &["opencode"],
        env_vars: &["OPENCODE_CLIENT"],
    },
    AgentEntry {
        name: "claude-code",
        process_names: &["claude"],
        env_vars: &["CLAUDECODE", "CLAUDE_CODE"],
    },
    AgentEntry {
        name: "cline",
        process_names: &[],
        env_vars: &["CLINE_ACTIVE"],
    },
    AgentEntry {
        name: "goose",
        process_names: &["goose"],
        env_vars: &["GOOSE_TERMINAL"],
    },
    AgentEntry {
        name: "openclaw",
        process_names: &[],
        env_vars: &["OPENCLAW_SHELL"],
    },
    AgentEntry {
        name: "pi",
        process_names: &[],
        env_vars: &["PI_CODING_AGENT"],
    },
    AgentEntry {
        name: "roo-code",
        process_names: &[],
        env_vars: &["ROO_ACTIVE"],
    },
    AgentEntry {
        name: "trae",
        process_names: &["trae"],
        env_vars: &["TRAE_AI_SHELL_ID"],
    },
    AgentEntry {
        name: "replit",
        process_names: &[],
        env_vars: &["REPL_ID"],
    },
    AgentEntry {
        name: "github-copilot",
        process_names: &["copilot"],
        env_vars: &["COPILOT_MODEL", "COPILOT_ALLOW_ALL", "COPILOT_GITHUB_TOKEN"],
    },
    AgentEntry {
        name: "aider",
        process_names: &["aider"],
        env_vars: &[],
    },
    AgentEntry {
        name: "carapace",
        process_names: &["cara"],
        env_vars: &[],
    },
    AgentEntry {
        name: "codebuddy",
        process_names: &["codebuddy"],
        env_vars: &[],
    },
    AgentEntry {
        name: "devin",
        process_names: &["devin"],
        env_vars: &[],
    },
    AgentEntry {
        name: "gloamy",
        process_names: &["gloamy"],
        env_vars: &[],
    },
    AgentEntry {
        name: "hermes",
        process_names: &["hermes"],
        env_vars: &[],
    },
    AgentEntry {
        name: "ironclaw",
        process_names: &["ironclaw"],
        env_vars: &[],
    },
    AgentEntry {
        name: "kimi-cli",
        process_names: &["kimi", "kimi-cli"],
        env_vars: &[],
    },
    AgentEntry {
        name: "loong",
        process_names: &["loong"],
        env_vars: &[],
    },
    AgentEntry {
        name: "microclaw",
        process_names: &["microclaw"],
        env_vars: &[],
    },
    AgentEntry {
        name: "moltis",
        process_names: &["moltis"],
        env_vars: &[],
    },
    AgentEntry {
        name: "nanobot",
        process_names: &["nanobot"],
        env_vars: &[],
    },
    AgentEntry {
        name: "picoclaw",
        process_names: &["picoclaw"],
        env_vars: &[],
    },
    AgentEntry {
        name: "windsurf",
        process_names: &["windsurf"],
        env_vars: &[],
    },
    AgentEntry {
        name: "zeroclaw",
        process_names: &["zeroclaw"],
        env_vars: &[],
    },
];
