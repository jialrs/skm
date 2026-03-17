# GEMINI.md - Project Context for SKM (Skill Manager)

This document provides essential context and instructions for AI agents working on the `skm` project.

## Project Overview

`skm` is a lightweight, high-performance CLI tool written in **Rust** designed to manage and synchronize **Claude Agent Skills** (`SKILL.md` format) across multiple machines. It treats skills as "Infrastructure as Code," utilizing Git for versioning and symlinks for deployment to the local environment.

### Core Concepts

*   **The Vault**: A local Git repository (default: `~/skm/vault`) containing all skill folders.
*   **The Target**: The official Claude skills directory (default: `~/.claude/skills/`).
*   **Deployment**: Skills are symlinked from the Vault to the Target, allowing real-time updates and Git tracking.
*   **Runtime**: Leverages `uv run` within scripts associated with `SKILL.md` for zero-config environment setup.

## Building and Running

Since this is a Rust project, the standard Cargo commands apply:

*   **Build**: `cargo build`
*   **Run**: `cargo run -- [args]`
*   **Test**: `cargo test`
*   **Check**: `cargo check`

## Development Conventions

### Documentation Synchronization
*   **Mandatory Updates**: All code changes, feature additions, or architectural shifts must be synchronized in both `README.md` and `TODO.md`.
*   **Consistency**: These files must always reflect the current, accurate state of the project.

### Tech Stack & Libraries
*   **Language**: Rust (2024 edition).
*   **CLI Parsing**: Use `clap`.
*   **Serialization/Deserialization**: Use `serde` for `config.json` or YAML frontmatter.
*   **File System Operations**: 
    *   Use `walkdir` for traversing the vault.
    *   Use `std::os::unix::fs::symlink` for creating symlinks (primary target: macOS/Linux).

### Skill Structure
Managed skills should follow this directory structure:
```text
my-skill-folder/
├── SKILL.md          # The instruction set for Claude
├── scripts/          # Python/Bash scripts
│   └── main.py       # Use PEP 723 for dependency metadata
└── config.json       # Optional metadata (tags, platform affinity, etc.)
```

### Implementation Priorities
1.  **"Clean" Linker**: When linking, ensure the tool validates source paths, handles existing/dead links in the Target directory, and cleans up appropriately.
2.  **Dry Run Mode**: Always implement a `--dry-run` flag for commands that modify the file system (especially `link` and `sync`) to prevent accidental data loss.
3.  **Cross-Platform Considerations**: While the focus is macOS/Linux (Unix symlinks), consider future-proofing for Windows symlink support if needed.

## Key Files
*   `README.md`: Contains the original technical specification and architectural flow.
*   `Cargo.toml`: Project metadata and dependencies.
*   `src/main.rs`: Entry point for the CLI tool.
