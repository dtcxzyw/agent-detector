# agent-detector

[![CI](https://github.com/dtcxzyw/agent-detector/actions/workflows/ci.yml/badge.svg)](https://github.com/dtcxzyw/agent-detector/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/agent-detector)](https://crates.io/crates/agent-detector)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/agent-detector)](https://crates.io/crates/agent-detector)
[![docs.rs](https://img.shields.io/docsrs/agent-detector)](https://docs.rs/agent-detector)

An agent detector for Rust CLI applications.

## Motivation

The Rust CLI application ecosystem is rich and diverse. AI coding agents
are increasingly using CLI tools to process data. However, typical CLI
output is optimized for humans — colors, emoji, progress spinners,
interactive prompts, and unstructured text — all of which are unfriendly
to agent consumption. This crate detects whether an agent is the caller
by inspecting the parent process name and relevant environment
variables, allowing CLIs to automatically switch to agent-friendly output
formats.

## Installation

```bash
cargo add agent-detector
```

## Usage

```rust
use agent_detector::{detect, is_agent, agent_name};

// Check if running under an agent
if is_agent() {
    eprintln!("agent detected");
}

// Get agent details
if let Some(info) = detect() {
    println!("agent: {} ({:?})", info.name, info.source);
}

// Just the name
let name = agent_name(); // Option<String>
```

## CLI

A `whichagent` binary is also included:

```bash
$ cargo install agent-detector
$ whichagent
opencode

$ whichagent && echo "detected" || echo "not detected"
detected
```

## API

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentInfo {
    pub name: String,
    pub source: DetectionSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionSource {
    ParentProcess,    // detected via parent process tree (requires `process-tree` feature, enabled by default)
    StandardEnvVar,   // detected via AI_AGENT / AGENT
    ToolEnvVar,       // detected via tool-specific env var
}

pub fn detect() -> Option<AgentInfo>;
pub fn is_agent() -> bool;
pub fn agent_name() -> Option<String>;
```

## Detection Strategy

Three tiers, checked in priority order:

1. **Parent process tree** — walks ancestor processes, matching known agent binary names and launcher/script names (covers interpreter-based CLIs). Most accurate, especially for agent-calling-agent scenarios.

2. **`AI_AGENT` / `AGENT`** — standard environment variables (per [agents.md #136](https://github.com/agentsmd/agents.md/issues/136)). The value is lowercased; unknown values are still treated as agent detections. Generic boolean markers (for example `AGENT=1`) are deferred until after tool-specific env vars so that a precise agent name wins.

3. **Tool-specific env vars** — checks for environment variables set by specific agents.

## Supported Agents

| Agent | L1 (Process) | L3 (Tool Env Var) |
|-------|:---:|:---|
| Aider | aider | — |
| AlayaCore | alayacore | — |
| Amp | amp | AMP_CURRENT_THREAD_ID |
| Anda Bot | anda | — |
| Antigravity | antigravity | ANTIGRAVITY_AGENT, ANTIGRAVITY_CLI_ALIAS |
| AstrBot | astrbot | ASTRBOT_CLI |
| Augment CLI | augment-cli | AUGMENT_AGENT |
| Autohand Code | autohand | AUTOHAND_CLI |
| Axiomate | axiomate | — |
| Bash Agent | — | BASH_AGENT_HOME |
| Carapace | cara | — |
| Claude Code | claude | CLAUDECODE, CLAUDE_CODE |
| ClawX | clawx | CLAWHUB_WORKDIR |
| Cline | cline | CLINE_ACTIVE |
| CodeBuddy | codebuddy | — |
| Codebuff | codebuff, freebuff, codebuff-tui | — |
| CodeProxy CLI | codeproxy | — |
| Codex CLI | codex | CODEX_SANDBOX, CODEX_CI, CODEX_THREAD_ID, CODEX_SANDBOX_NETWORK_DISABLED |
| Continue CLI | cn | — |
| Cow Agent | cow | — |
| Cowork | claude¹ | CLAUDE_CODE_IS_COWORK + (CLAUDECODE\|CLAUDE_CODE) |
| Crush | crush | — |
| Ctrl | ctrl | — |
| Cursor | cursor | CURSOR_TRACE_ID |
| Cursor CLI | cursor-agent | CURSOR_AGENT, CURSOR_EXTENSION_HOST_ROLE |
| Deep Code | deepcode | — |
| Deep Copilot | deep-copilot | — |
| DeepLossless | deeplossless | — |
| DeepSeek Harness | dsh | DSH_SHELL |
| DeepSeek-TUI | deepseek-tui | — |
| DeepSeekX | deepseekx | — |
| Devin | devin | — |
| dscli | dscli | — |
| dscode | dscode | — |
| Factory Droid | droid | — |
| Frontier Agent | apodex, frontier-agent | APODEX_SESSION_ID |
| Fuxi | fuxi | — |
| Gemini CLI | gemini | GEMINI_CLI |
| GitHub Copilot | copilot | COPILOT_MODEL, COPILOT_ALLOW_ALL, COPILOT_GITHUB_TOKEN |
| Gloamy | gloamy | — |
| GoAgent | goagent | — |
| Goose | goose | GOOSE_TERMINAL |
| Grok Build | grok | GROK_PLUGIN_ROOT, GROK_PLUGIN_DATA |
| HalfCopilot | halfcopilot | — |
| Hermes Agent | hermes | — |
| IronClaw | ironclaw | — |
| Junie | junie | JUNIE_DATA, JUNIE_SHIM_PATH |
| Kilo Code | kilo | — |
| Kimi CLI | kimi, kimi-cli | KIMI_PLUGIN_ROOT |
| KIMIX | kimix | — |
| Kiro | kiro-cli | — |
| LangBot | langbot | — |
| Langcli | langcli | — |
| Loong | loong | — |
| Loushang | loushang | — |
| Markus | markus | — |
| MastraCode | mastracode | — |
| MicroClaw | microclaw | — |
| MiMo Code | mimo | MIMOCODE |
| MiniMax Code | mcode, mcode-tools | — |
| Mistral Vibe | vibe | — |
| Moltis | moltis | — |
| Morph | mistermorph | — |
| Muse Code | muse | — |
| Nanobot | nanobot | — |
| Octofriend | octofriend, octo | — |
| Oh My Pi | omp | — |
| Omnigent | omnigent | — |
| Open Interpreter | interpreter | — |
| OpenClaw | — | OPENCLAW_SHELL |
| OpenCode | opencode | OPENCODE_CLIENT, OPENCODE |
| OpenHands | openhands, openhands-agent, openhands-agent-server | — |
| Operit | operit | — |
| Pi | — | PI_CODING_AGENT |
| PicoClaw | picoclaw | — |
| Prime Agent | prime-agent | — |
| Proma | proma | — |
| Qoder | qodercli | — |
| Qwen Code | qwen | QWEN_CODE |
| Reasonix | reasonix | — |
| Replit | — | REPL_ID |
| Roo Code | — | ROO_ACTIVE |
| Snow CLI | snow | — |
| SolonCode | soloncode | — |
| SWE-agent | sweagent | — |
| Tau | tau | — |
| Tday | tday | — |
| Tiangong | tiangong | — |
| TRAE AI | trae | TRAE_AI_SHELL_ID |
| Waku | waku | — |
| Warp | — | OZ_RUN_ID |
| Whale | whale | — |
| WhipCode | whipcode | — |
| Windsurf | windsurf | — |
| Xpro | xpro | — |
| ZCode | zcode | — |
| Zero | zero | — |
| ZeroClaw | zeroclaw | — |
| zot | zot | — |

¹ Detected via `claude` process tree match when `CLAUDE_CODE_IS_COWORK` is also set.

## License

Licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome. Please ensure `pre-commit install` is run and
all hooks pass before submitting a PR.

To add support for a new agent:
1. Add the agent to the `AGENTS` table in `src/agents.rs`.
2. Add a test in `src/lib.rs` verifying the new detection logic.
3. Update the Supported Agents table in this README.
4. Run `cargo clippy && cargo test && cargo test --no-default-features`.

## Inspired by

- [vercel/detect-agent](https://github.com/vercel/vercel/tree/main/packages/detect-agent)
- [huggingface/huggingface_hub](https://github.com/huggingface/huggingface_hub/blob/main/src/huggingface_hub/utils/_detect_agent.py)
