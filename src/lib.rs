use async_trait::async_trait;
use std::{error::Error, process::Output};

pub mod localrunner;
pub mod openssh_runner;
pub mod shfile;

#[async_trait]
pub trait CmdRunner {
    type Error: Error + Send + Sync + 'static;

    async fn run(&self, cmd: &str, args: &[&str]) -> Result<Output, Self::Error>;
}
