use async_trait::async_trait;
use std::process::Output;

pub mod localrunner;
pub mod openssh_runner;
pub mod shfile;

#[async_trait]
pub trait CmdRunner: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, Self::Error>;
}

#[async_trait]
pub trait ReadAll {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn read_all(&self) -> Result<Vec<u8>, Self::Error>;
}
