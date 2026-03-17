# SKM Implementation TODO

Detailed roadmap for building the Skill Manager CLI in Rust.

## Phase 1: Project Setup & CLI Foundation
- [x] **Dependencies**: Update `Cargo.toml` with essential crates.
- [x] **CLI Skeleton**: Define the command structure in `src/main.rs`.
- [x] **Global Flags**: Implement the `--dry-run` and `--target-dir` flags.

## Phase 2: Configuration & Path Management
- [x] **Configuration Schema**: Define a struct to store paths and remote URL.
- [x] **Persistence**: Implement loading/saving to `~/skm/config.toml`.
- [x] **Path Resolution**: Create a module to handle default paths.

## Phase 3: Core Commands Implementation
### `skm onboard`
- [x] Implement interactive prompts for paths and Git URL using `dialoguer`.
- [x] Save configuration and trigger `init` logic.

### `skm init`
- [x] Implement Git cloning logic.
- [x] Save the `remote_url` to the configuration file.

### `skm push`
- [x] List changed files in the vault.
- [x] Interactively prompt for commit message and confirmation.
- [x] Implement git add, commit, and push.

### `skm list`
- [x] Traverse the Vault directory to find skills (folders with `SKILL.md`).
- [x] Detect if skills are already linked in the Target directory.
- [x] Display a formatted table or list with status (Linked/Unlinked).

### `skm check`
- [x] Check for `git` installation.
- [x] Check for `uv` installation.

## Phase 4: The "Clean" Linker (Critical Logic)
- [x] **Link Discovery**: Map Vault skills to their intended Target symlink names.
- [x] **Conflict Handling**:
  - [x] If Target exists but is a "dead" link, remove it.
  - [x] If Target exists and is a regular file/dir, warn the user.
- [x] **Symlink Creation**: Use `std::os::unix::fs::symlink`.
- [x] **Dry Run Logic**: Print intended actions without executing them.

## Phase 5: Synchronization & Advanced Features
### `skm pull`
- [x] Implement `git pull` in the Vault directory.
- [x] Automatically trigger a `link` validation after pull.

### `skm link [skill_name]`
- [x] Support linking a specific skill by name or all skills.

### `skm unlink [skill_name]`
- [x] Implement removing specific or all managed symlinks.

### `skm completion <shell>`
- [x] Implement completion script generation using `clap_complete`.
- [x] Support Bash, Zsh, and Fish shells.

### `skm config`
- [x] Implement viewing of current configuration.

## Phase 6: Refinement & Testing
- [x] **Cross-platform Support**: Implement cross-platform symlinking (Windows support).
- [x] **Error Handling**: Use structured `anyhow` errors throughout.
- [x] **Unit Tests**:
  - [x] Test path resolution logic.
  - [x] Test symlink validation logic.
  - [x] Test configuration serialization.
- [x] **Integration Tests**: Added tests for help, check, config, link dry-run, and list uninitialized.
- [x] **Documentation**: Keep `README.md` and `TODO.md` updated.
