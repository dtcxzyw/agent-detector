# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/dtcxzyw/agent-detector/compare/v0.1.0...v0.2.0) - 2026-04-29

### Fixed

- add missing release-pr job to workflow
- trim env var whitespace in is_cowork_override and env_var_is_set
- skip current process in parent tree traversal, add AGENT empty value tests
- use RAII guard for test env vars and reject empty env values consistently
- correct agent data and simplify cowork detection
- process tree detection on Windows (.exe suffix and case sensitivity)
- set release_always=false to only publish on release PR merge
- correct release-plz.toml syntax (invalid top-level key)

### Other

- deny missing_safety_doc, missing_panics_doc, missing_errors_doc clippy lints
- deny undocumented_unsafe_blocks clippy lint
- deduplicate cowork logic, add #[non_exhaustive], improve docs
- agent names are always returned lowercase
- merge agent tables and fix detection edge cases
- remove redundant L2 (Standard) column from Supported Agents table
- add docs.rs badge to README
- add crates.io version badge to README
- remove empty [changelog] section from release-plz.toml
