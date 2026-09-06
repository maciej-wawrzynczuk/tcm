use thiserror::Error;
use tokio::process::Command;

// Do I need stderr in normal situations?
pub trait CmdRunner {
    async fn run(&self, cmd: &[&str]) -> Result<String, RunError>;
}

pub struct LocalCmdRunner {}

impl CmdRunner for LocalCmdRunner {
    async fn run(&self, cmd: &[&str]) -> Result<String, RunError> {
        let cmd0 = cmd.first().ok_or(RunError::NoCommandProvided)?;
        let args = &cmd[1..];
        let o = Command::new(cmd0).args(args).output().await?;
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

#[derive(Debug, Error)]
pub enum RunError {
    #[error("process spawn error {0}")]
    SpawnFailed(#[from] std::io::Error),
    #[error("no command provided")]
    NoCommandProvided,
    #[error("command failed witch {code}, {stderr}")]
    CommandFailed { code: i32, stderr: String },
}
