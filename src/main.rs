use serde::Serialize;
use std::process::Command;
use thiserror::Error;

fn main() -> color_eyre::eyre::Result<()> {
    let f = ShFile::claim_existing("/etc/hosts")?;
    let s = toml::to_string(&f)?;
    println!("{s}");

    Ok(())
}

#[derive(Serialize)]
struct ShFile {
    path: String,
}

// Need to find a better way to execute shells
// Subject can't in dependent on execution.
// Use DI

// Do I need stderr in normal situations?
trait CmdRunner {
    fn run(cmd: &[&str]) -> Result<String, RunError>;
}

struct LocalCmdRunner {}

impl CmdRunner for LocalCmdRunner {
    fn run(cmd: &[&str]) -> Result<String, RunError> {
        let cmd0 = cmd.first().ok_or(RunError("no command provided".into()))?;
        let args = &cmd[1..];
        let o = Command::new(cmd0)
            .args(args);

        Err(RunError("Not yet".into()))
    }
}

impl ShFile {
    fn claim_existing(path: &str) -> Result<Self, ClaimError> {
        let o = Command::new("stat")
            .arg(path)
            .arg("-c")
            .arg("%F")
            .output()
            .expect("stat failed");

        let stdout = String::from_utf8_lossy(&o.stdout).to_string();
        let stdout = stdout.trim();

        match stdout {
            "regular file" => Ok(Self {
                path: path.to_string(),
            }),
            _ => Err(ClaimError(format!("I need a regular file, not {}", stdout))),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unable to claim {0}")]
struct ClaimError(String);

#[derive(Debug, Error)]
#[error("Run error {0}")]
struct RunError(String);
