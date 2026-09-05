use serde::Serialize;
use std::process::Command;
use thiserror::Error;

fn main() -> color_eyre::eyre::Result<()> {
    let f = ShFile::claim_existing("/etc/hosts")?;
    let s = toml::to_string(&f)?;
    println!("{s}");

    let l = LocalCmdRunner{};
    let ll = l.run(&["ls"])?;
    print!("the result: {ll}");

    Ok(())
}

#[derive(Serialize)]
struct ShFile {
    path: String,
}

// Do I need stderr in normal situations?
trait CmdRunner {
    fn run(&self, cmd: &[&str]) -> Result<String, RunError>;
}

struct LocalCmdRunner {}

impl CmdRunner for LocalCmdRunner {
    fn run(&self, cmd: &[&str]) -> Result<String, RunError> {
        let cmd0 = cmd.first().ok_or(RunError::NoCommandProvided)?;
        let args = &cmd[1..];
        let o = Command::new(cmd0).args(args).output()?;
        if o.status.success() {
            Ok(String::from_utf8_lossy(&o.stdout).into_owned())
        } else {
            Err(RunError::CommandFailed {
                code: o.status.code().unwrap_or(-1), // None if process terminated by signal. TODO:
                                                     // Handle it better
                stderr: String::from_utf8_lossy(&o.stderr).into_owned(),
            })
        }
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
enum RunError {
    #[error("process spawn error {0}")]
    SpawnFailed(#[from] std::io::Error),
    #[error("no command provided")]
    NoCommandProvided,
    #[error("command failed witch {code}, {stderr}")]
    CommandFailed { code: i32, stderr: String },
}
