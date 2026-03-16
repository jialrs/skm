use anyhow::Result;
use std::process::Command;

pub fn run() -> Result<()> {
    println!("Checking environment...");
    check_tool("git", "--version")?;
    check_tool("uv", "--version")?;
    Ok(())
}

fn check_tool(tool: &str, arg: &str) -> Result<()> {
    match Command::new(tool).arg(arg).output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("✅ {} found: {}", tool, version);
            } else {
                println!("❌ {} found but returned error: {}", tool, String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(_) => {
            println!("❌ {} not found in PATH", tool);
            match tool {
                "git" => {
                    println!("   💡 Install git: 'brew install git' (macOS) or 'apt-get install git' (Linux)");
                }
                "uv" => {
                    println!("   💡 Install uv: 'brew install uv' (macOS) or 'curl -LsSf https://astral.sh/uv/install.sh | sh' (Linux/macOS)");
                }
                _ => {}
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_tool() {
        // Assume git is installed on test env
        assert!(check_tool("git", "--version").is_ok());
        // Assume non-existent tool
        assert!(check_tool("nonexistenttool123", "--version").is_ok()); // Should not error, just print
    }
}
