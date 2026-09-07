use async_trait::async_trait;
use thiserror::Error;
pub mod localrunner;
pub mod shfile;

#[async_trait]
pub trait CmdRunner {
    async fn run(&self, cmd: &[&str]) -> Result<String, RunError>;
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
