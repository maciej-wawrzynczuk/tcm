use crate::{CmdRunner, RunError};
use async_trait::async_trait;
use openssh::{KnownHosts, Session};
use std::process::Output;

pub struct SSHRunner {
    session: Session,
}

impl SSHRunner {
    pub async fn new(hostname: &str) -> Result<Self, openssh::Error> {
        let session = openssh::Session::connect(hostname, KnownHosts::Strict).await?;

        Ok(Self { session })
    }
}

#[async_trait]
impl CmdRunner for SSHRunner {
    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, RunError> {
        let o = self.session
            .command(cmd)
            .args(args)
            .output()
            .await?;
        Ok(o)
    }
}
