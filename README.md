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

## Inspired by

- [vercel/detect-agent](https://github.com/vercel/vercel/tree/main/packages/detect-agent)
- [huggingface/huggingface_hub](https://github.com/huggingface/huggingface_hub/blob/main/src/huggingface_hub/utils/_detect_agent.py)
