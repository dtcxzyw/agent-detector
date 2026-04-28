# agent-detector

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
    ParentProcess,    // detected via parent process tree
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

2. **`AI_AGENT` / `AGENT`** — standard environment variables (per [agents.md #136](https://github.com/agentsmd/agents.md/issues/136)). The value is used as-is; unknown values are still treated as agent detections.

3. **Tool-specific env vars** — checks for environment variables set by specific agents.

## Supported Agents

| Agent | L1 (Process) | L2 (Standard) | L3 (Tool Env Var) |
|-------|:---:|:---:|:---|
| Antigravity | amp | AI_AGENT/AGENT | ANTIGRAVITY_AGENT |
| Augment CLI | augment-cli | AI_AGENT/AGENT | AUGMENT_AGENT |
| Claude Code | claude | AI_AGENT/AGENT | CLAUDECODE, CLAUDE_CODE |
| Cline | — | AI_AGENT/AGENT | CLINE_ACTIVE |
| Codex CLI | codex | AI_AGENT/AGENT | CODEX_SANDBOX, CODEX_CI, CODEX_THREAD_ID |
| Cowork | — | AI_AGENT/AGENT | CLAUDE_CODE_IS_COWORK + (CLAUDECODE\|CLAUDE_CODE) |
| Cursor | cursor | AI_AGENT/AGENT | CURSOR_TRACE_ID |
| Cursor CLI | — | AI_AGENT/AGENT | CURSOR_AGENT, CURSOR_EXTENSION_HOST_ROLE |
| Devin | devin | AI_AGENT/AGENT | — |
| Gemini CLI | gemini | AI_AGENT/AGENT | GEMINI_CLI |
| GitHub Copilot | github-copilot | AI_AGENT/AGENT | COPILOT_MODEL, COPILOT_ALLOW_ALL, COPILOT_GITHUB_TOKEN |
| Goose | goose | AI_AGENT/AGENT | GOOSE_TERMINAL |
| OpenClaw | — | AI_AGENT/AGENT | OPENCLAW_SHELL |
| OpenCode | opencode | AI_AGENT/AGENT | OPENCODE_CLIENT |
| Pi | — | AI_AGENT/AGENT | PI_CODING_AGENT |
| Replit | — | AI_AGENT/AGENT | REPL_ID |
| Roo Code | — | AI_AGENT/AGENT | ROO_ACTIVE |
| TRAE AI | trae | AI_AGENT/AGENT | TRAE_AI_SHELL_ID |
| Windsurf | windsurf | AI_AGENT/AGENT | — |

## Inspired by

- [vercel/detect-agent](https://github.com/vercel/vercel/tree/main/packages/detect-agent)
- [huggingface/huggingface_hub](https://github.com/huggingface/huggingface_hub/blob/main/src/huggingface_hub/utils/_detect_agent.py)
