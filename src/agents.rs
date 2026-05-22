#[derive(Debug)]
pub(crate) struct AgentEntry {
    pub name: &'static str,
    // When `process-tree` is off, `process_names` is unused but still
    // compiled. The data is a few hundred bytes of static string slices; the
    // noise of `#[cfg]` on every entry outweighs the negligible binary size
    // savings.
    #[cfg_attr(not(feature = "process-tree"), allow(dead_code))]
    pub process_names: &'static [&'static str],
    pub env_vars: &'static [&'static str],
}

// Order determines L3 priority: earlier entries are checked first.
// Agents with overlapping env vars are not expected in practice; if they
// occur, the first match wins.
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
    AgentEntry {
        name: "alayacore",
        process_names: &["alayacore"],
        env_vars: &[],
    },
    AgentEntry {
        name: "anda-bot",
        process_names: &["anda"],
        env_vars: &[],
    },
    AgentEntry {
        name: "astrbot",
        process_names: &["astrbot"],
        env_vars: &["ASTRBOT_CLI"],
    },
    AgentEntry {
        name: "autohand-code",
        process_names: &["autohand"],
        env_vars: &["AUTOHAND_CLI"],
    },
    AgentEntry {
        name: "axiomate",
        process_names: &["axiomate"],
        env_vars: &[],
    },
    AgentEntry {
        name: "bash-agent",
        process_names: &[],
        env_vars: &["BASH_AGENT_HOME"],
    },
    AgentEntry {
        name: "clawx",
        process_names: &["clawx"],
        env_vars: &["CLAWHUB_WORKDIR"],
    },
    AgentEntry {
        name: "codeproxy-cli",
        process_names: &["codeproxy"],
        env_vars: &[],
    },
    AgentEntry {
        name: "cow-agent",
        process_names: &["cow"],
        env_vars: &[],
    },
    AgentEntry {
        name: "crush",
        process_names: &["crush"],
        env_vars: &[],
    },
    AgentEntry {
        name: "ctrl",
        process_names: &["ctrl"],
        env_vars: &[],
    },
    AgentEntry {
        name: "deep-code",
        process_names: &["deepcode"],
        env_vars: &[],
    },
    AgentEntry {
        name: "deep-copilot",
        process_names: &["deep-copilot"],
        env_vars: &[],
    },
    AgentEntry {
        name: "deeplossless",
        process_names: &["deeplossless"],
        env_vars: &[],
    },
    AgentEntry {
        name: "deepseek-tui",
        process_names: &["deepseek-tui"],
        env_vars: &[],
    },
    AgentEntry {
        name: "deepseekx",
        process_names: &["deepseekx"],
        env_vars: &[],
    },
    AgentEntry {
        name: "dscli",
        process_names: &["dscli"],
        env_vars: &[],
    },
    AgentEntry {
        name: "dscode",
        process_names: &["dscode"],
        env_vars: &[],
    },
    AgentEntry {
        name: "goagent",
        process_names: &["goagent"],
        env_vars: &[],
    },
    AgentEntry {
        name: "halfcopilot",
        process_names: &["halfcopilot"],
        env_vars: &[],
    },
    AgentEntry {
        name: "kilo-code",
        process_names: &["kilo"],
        env_vars: &[],
    },
    AgentEntry {
        name: "kimix",
        process_names: &["kimix"],
        env_vars: &[],
    },
    AgentEntry {
        name: "langbot",
        process_names: &["langbot"],
        env_vars: &[],
    },
    AgentEntry {
        name: "langcli",
        process_names: &["langcli"],
        env_vars: &[],
    },
    AgentEntry {
        name: "markus",
        process_names: &["markus"],
        env_vars: &[],
    },
    AgentEntry {
        name: "morph",
        process_names: &["mistermorph"],
        env_vars: &[],
    },
    AgentEntry {
        name: "oh-my-pi",
        process_names: &["omp"],
        env_vars: &[],
    },
    AgentEntry {
        name: "operit",
        process_names: &["operit"],
        env_vars: &[],
    },
    AgentEntry {
        name: "proma",
        process_names: &["proma"],
        env_vars: &[],
    },
    AgentEntry {
        name: "qwen-code",
        process_names: &["qwen"],
        env_vars: &[],
    },
    AgentEntry {
        name: "reasonix",
        process_names: &["reasonix"],
        env_vars: &[],
    },
    AgentEntry {
        name: "snow-cli",
        process_names: &["snow"],
        env_vars: &[],
    },
    AgentEntry {
        name: "soloncode",
        process_names: &["soloncode"],
        env_vars: &[],
    },
    AgentEntry {
        name: "tday",
        process_names: &["tday"],
        env_vars: &[],
    },
    AgentEntry {
        name: "tiangong",
        process_names: &["tiangong"],
        env_vars: &[],
    },
    AgentEntry {
        name: "whale",
        process_names: &["whale"],
        env_vars: &[],
    },
    AgentEntry {
        name: "xpro",
        process_names: &["xpro"],
        env_vars: &[],
    },
    AgentEntry {
        name: "zot",
        process_names: &["zot"],
        env_vars: &[],
    },
];
