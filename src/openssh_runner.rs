use crate::{CmdRunner, RunError};
use async_trait::async_trait;
use openssh::{KnownHosts, Session, SessionBuilder};
use std::process::Output;

pub struct SSHRunner {
    session: Session,
}

pub struct SSHRunnerBuilder {
    sb: SessionBuilder,
    host: String,
}

impl Default for SSHRunnerBuilder {
    fn default() -> Self {
        let sb = SessionBuilder::default();
        let host = String::default();
        Self { sb, host }
    }
}

impl SSHRunnerBuilder {
    pub fn host(mut self, h: String) -> Self {
        self.host = h;
        self
    }

    pub fn user(mut self, u: String) -> Self {
        self.sb.user(u);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.sb.port(port);
        self
    }

    pub fn ignore_key(mut self) -> Self {
        self.sb.known_hosts_check(KnownHosts::Accept);
        self
    }

    pub async fn build(self) -> Result<SSHRunner, openssh::Error> {
        let session = self.sb.connect(self.host).await?;
        Ok(SSHRunner { session })
    }
}

#[async_trait]
impl CmdRunner for SSHRunner {
    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, RunError> {
        let o = self.session.command(cmd).args(args).output().await?;
        Ok(o)
    }
}
