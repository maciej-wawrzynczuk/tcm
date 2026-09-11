use crate::CmdRunner;
use crate::RunError;
use async_trait::async_trait;
use std::process::Output;
use tokio::process::Command;

pub struct LocalCmdRunner {}

#[async_trait]
impl CmdRunner for LocalCmdRunner {
    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, RunError> {
        let o = Command::new(cmd).args(args).output().await?;
        Ok(o)
    }
}
