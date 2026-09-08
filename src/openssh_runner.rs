use crate::{CmdRunner, RunError};
use async_trait::async_trait;
use openssh::{KnownHosts, Session};

pub struct SSHRunner {
    session: Session,
}

impl SSHRunner {
    pub async fn new(hostname: &str) -> Result<Self, openssh::Error> {
        let session = openssh::Session::connect(hostname, KnownHosts::Strict).await?;

        Ok(Self { session })
    }
}

