#[cfg(feature = "process-tree")]
pub static PARENT_PROCESS_NAMES: &[(&str, &str)] = &[
    ("aider", "aider"),
    ("amp", "amp"),
    ("antigravity", "antigravity"),
    ("augment-cli", "augment-cli"),
    ("claude", "claude-code"),
    ("codebuddy", "codebuddy"),
    ("codex", "codex"),
    ("cursor", "cursor"),
    ("devin", "devin"),
    ("gemini", "gemini"),
    ("github-copilot", "github-copilot"),
    ("goose", "goose"),
    ("kimi", "kimi-cli"),
    ("kimi-cli", "kimi-cli"),
    ("opencode", "opencode"),
    ("trae", "trae"),
    ("windsurf", "windsurf"),
];

#[derive(Debug)]
pub struct ToolAgent {
    pub name: &'static str,
    pub env_vars: &'static [&'static str],
}

pub static TOOL_AGENTS: &[ToolAgent] = &[
    ToolAgent {
        name: "cursor",
        env_vars: &["CURSOR_TRACE_ID"],
    },
    ToolAgent {
        name: "cursor-cli",
        env_vars: &["CURSOR_AGENT", "CURSOR_EXTENSION_HOST_ROLE"],
    },
    ToolAgent {
        name: "gemini",
        env_vars: &["GEMINI_CLI"],
    },
    ToolAgent {
        name: "codex",
        env_vars: &["CODEX_SANDBOX", "CODEX_CI", "CODEX_THREAD_ID"],
    },
    ToolAgent {
        name: "antigravity",
        env_vars: &["ANTIGRAVITY_AGENT"],
    },
    ToolAgent {
        name: "augment-cli",
        env_vars: &["AUGMENT_AGENT"],
    },
    ToolAgent {
        name: "opencode",
        env_vars: &["OPENCODE_CLIENT"],
    },
    ToolAgent {
        name: "cowork",
        env_vars: &["CLAUDE_CODE_IS_COWORK"],
    },
    ToolAgent {
        name: "claude-code",
        env_vars: &["CLAUDECODE", "CLAUDE_CODE"],
    },
    ToolAgent {
        name: "cline",
        env_vars: &["CLINE_ACTIVE"],
    },
    ToolAgent {
        name: "goose",
        env_vars: &["GOOSE_TERMINAL"],
    },
    ToolAgent {
        name: "openclaw",
        env_vars: &["OPENCLAW_SHELL"],
    },
    ToolAgent {
        name: "pi",
        env_vars: &["PI_CODING_AGENT"],
    },
    ToolAgent {
        name: "roo-code",
        env_vars: &["ROO_ACTIVE"],
    },
    ToolAgent {
        name: "trae",
        env_vars: &["TRAE_AI_SHELL_ID"],
    },
    ToolAgent {
        name: "replit",
        env_vars: &["REPL_ID"],
    },
    ToolAgent {
        name: "github-copilot",
        env_vars: &["COPILOT_MODEL", "COPILOT_ALLOW_ALL", "COPILOT_GITHUB_TOKEN"],
    },
];
