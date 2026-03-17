# SKM (Skill Manager) - Technical Specification
## 1. Overview

`skm` is a lightweight, high-performance CLI tool written in **Rust** designed to manage and synchronize **Claude Agent Skills** (`SKILL.md` format) across multiple machines. It treats skills as "Infrastructure as Code," using Git for versioning and symlinks for deployment.

### Prerequisites

To use `skm` effectively, ensure you have the following tools installed:

*   **Git**: Required for syncing your skill vault (`pull` and `push`).
*   **uv**: Required for the zero-config execution of Python-based skills (highly recommended).
*   **Rust (Cargo)**: Only needed if building from source.

---

## 2. Core Concepts

* **The Vault**: A local Git repository (default: `~/skm/vault`) containing all skill folders. The remote Git URL is stored in the configuration.
* **The Target**: The official Claude skills directory (default: `~/.claude/skills/`). This can be overridden via configuration or CLI flags.
* **Deployment**: Skills are **symlinked** from the Vault to the Target to allow real-time updates and Git tracking.
* **Runtime**: Leverages `uv run` within the scripts associated with `SKILL.md` to ensure zero-config environment setup on new machines.

---

## 3. Architecture & Flow

1. **Onboard/Init**: `skm onboard` or `skm init` sets up the configuration (Vault path, Target path, Git URL) and clones the repository.
2. **Pull**: `git pull` updates the Vault from the configured remote and validates links.
3. **Push**: Interactively commit and push local changes to the remote vault.
4. **Link**: Symlinks are created/validated in the Target directory.
5. **Execute**: Claude invokes the skill; `uv run` handles Python dependencies on-the-fly.

---

## 4. Proposed `SKILL.md` Structure

Every skill managed by `skm` should follow this standard:

```text
my-skill-folder/
├── SKILL.md          # The instruction set for Claude
├── scripts/          # Python/Bash scripts
│   └── main.py       # Use PEP 723 for dependency metadata
└── config.json       # Optional metadata for skm (e.g., tags, platform affinity)

```

---

## 5. CLI Commands

| Command | Description |
| --- | --- |
| `skm onboard` | Interactive setup for Vault path, Target path, and Git URL. |
| `skm init <repo_url>` | Non-interactive initialization (clones vault and saves URL). |
| `skm pull` | Pulls latest changes from Git and re-validates links. |
| `skm push` | Interactively commits and pushes changes to the remote vault. |
| `skm list` | Lists all available skills and their current status (Linked/Unlinked). |
| `skm link [skill_name]` | Symlinks specific or all skills. Supports `--target-dir` override. |
| `skm unlink [skill_name]` | Removes symlinks for specific or all skills. |
| `skm config` | View the current configuration. |
| `skm completion <shell>` | Generate shell completion scripts (Bash, Zsh, Fish). |
| `skm check` | Validates if `uv` and other required runtimes are installed. |

### Global Flags
*   `-d, --dry-run`: Preview changes without modifying the file system.
*   `--target-dir <path>`: Override the target directory for the current command.

---

## 6. Configuration

Configuration is stored in `~/skm/config.toml`:

```toml
vault_path = "/Users/user/skm/vault"
target_path = "/Users/user/.claude/skills"
remote_url = "https://github.com/username/my-skills.git"
```

---

## 7. Installation & Setup

### Automated Install (Recommended)
You can install `skm` using the automated install script:

**macOS/Linux (Shell):**
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/jialrs/skm/releases/latest/download/skm-installer.sh | sh
```

**Windows (PowerShell):**
```powershell
powershell -c "irm https://github.com/jialrs/skm/releases/latest/download/skm-installer.ps1 | iex"
```

### Homebrew (macOS/Linux)
You can install `skm` via Homebrew:
```bash
brew tap jialrs/skm
brew install skm
```

### Manual Build
1. **Build**: `cargo build --release`
2. **Setup**: Run `skm onboard` to configure your vault and target paths.
3. **Completion**: Add `source <(skm completion zsh)` to your `.zshrc` (or equivalent for your shell).

---

## 8. Implementation Details

### The "Clean" Linker
When running `skm link`, the tool:
1. Verifies the source path in the Vault.
2. Checks if a file/link already exists in the Target.
3. If it's a "dead" link (pointing to a non-existent vault path), it removes it.
4. Creates the new symlink.
5. Skips existing correct links.
