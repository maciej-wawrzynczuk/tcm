use async_trait::async_trait;
use crate::RunError;
use crate::CmdRunner;
use tokio::process::Command;
use std::process::Output;

// Do I need stderr in normal situations?
pub struct LocalCmdRunner {}

#[async_trait]
impl CmdRunner for LocalCmdRunner {
    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, RunError> {
        let o = Command::new(cmd).args(args).output().await?;
        Ok(o)
    }
}

