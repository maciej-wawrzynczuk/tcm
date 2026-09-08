use async_trait::async_trait;
use std::process::Output;
use thiserror::Error;

pub mod localrunner;
pub mod openssh_runner;
pub mod shfile;

#[async_trait]
pub trait CmdRunner {
    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, RunError>;
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
