# agent-detector

[![CI](https://github.com/dtcxzyw/agent-detector/actions/workflows/ci.yml/badge.svg)](https://github.com/dtcxzyw/agent-detector/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/agent-detector)](https://crates.io/crates/agent-detector)
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

```toml
[dependencies]
agent-detector = "0.1"
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

1. **Parent process tree** — walks ancestor processes, matches known agent binary names. Most accurate, especially for agent-calling-agent scenarios.

2. **`AI_AGENT` / `AGENT`** — standard environment variables (per [agents.md #136](https://github.com/agentsmd/agents.md/issues/136)). The value is lowercased; unknown values are still treated as agent detections.

3. **Tool-specific env vars** — checks for environment variables set by specific agents.

## Supported Agents

| Agent | L1 (Process) | L3 (Tool Env Var) |
|-------|:---:|:---|
| Aider | aider | — |
| Antigravity | amp | ANTIGRAVITY_AGENT |
| Augment CLI | augment-cli | AUGMENT_AGENT |
| Carapace | cara | — |
| Claude Code | claude | CLAUDECODE, CLAUDE_CODE |
| CodeBuddy | codebuddy | — |
| Cline | — | CLINE_ACTIVE |
| Codex CLI | codex | CODEX_SANDBOX, CODEX_CI, CODEX_THREAD_ID |
| Cowork | claude¹ | CLAUDE_CODE_IS_COWORK + (CLAUDECODE\|CLAUDE_CODE) |
| Cursor | cursor | CURSOR_TRACE_ID |
| Cursor CLI | — | CURSOR_AGENT, CURSOR_EXTENSION_HOST_ROLE |
| Devin | devin | — |
| Gemini CLI | gemini | GEMINI_CLI |
| GitHub Copilot | copilot | COPILOT_MODEL, COPILOT_ALLOW_ALL, COPILOT_GITHUB_TOKEN |
| Gloamy | gloamy | — |
| Goose | goose | GOOSE_TERMINAL |
| Hermes Agent | hermes | — |
| IronClaw | ironclaw | — |
| Kimi CLI | kimi, kimi-cli | — |
| Loong | loong | — |
| MicroClaw | microclaw | — |
| Moltis | moltis | — |
| Nanobot | nanobot | — |
| OpenClaw | — | OPENCLAW_SHELL |
| OpenCode | opencode | OPENCODE_CLIENT |
| Pi | — | PI_CODING_AGENT |
| PicoClaw | picoclaw | — |
| Replit | — | REPL_ID |
| Roo Code | — | ROO_ACTIVE |
| TRAE AI | trae | TRAE_AI_SHELL_ID |
| Windsurf | windsurf | — |
| ZeroClaw | zeroclaw | — |

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
