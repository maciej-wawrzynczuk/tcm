use crate::CmdRunner;
use async_trait::async_trait;
use std::process::Output;
use tokio::process::Command;

pub struct LocalCmdRunner {}

#[async_trait]
impl CmdRunner for LocalCmdRunner {
    type Error = std::io::Error;
    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, Self::Error> {
        let o = Command::new(cmd).args(args).output().await?;
        Ok(o)
    }
}
