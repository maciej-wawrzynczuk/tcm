use crate::{CmdRunner, RunError};
use thiserror::Error;

pub struct ShFile<T: CmdRunner> {
    r: T,
    path: String,
}

impl<T: CmdRunner> ShFile<T> {
    pub fn new(r: T) -> Self {
        Self {
            path: String::new(),
            r,
        }
    }

    pub async fn claim(mut self, filename: &str) -> Result<Self, ClaimError> {
        let cmd = "stat";
        let args = &["--format", "%F", filename];
        let o = self.r.run(cmd, args).await?;
        let stdout = String::from_utf8_lossy(&o.stdout);
        if stdout == "regular file" {
            self.path = filename.to_string();
            Ok(self)
        } else {
            Err(ClaimError::WrongFileType(stdout.into_owned()))
        }
    }

    pub async fn get_content(&self) -> Result<String, ContentError> {
        let cmd = "cat";
        let args = &[self.path.as_str()];
        let o = self.r.run(cmd, args).await?;
        let stdout = String::from_utf8_lossy(&o.stdout);
        Ok(stdout.into_owned())
    }
}

#[derive(Debug, Error)]
pub enum ClaimError {
    #[error("Unable to run command")]
    RunError(#[from] RunError),
    #[error("Wrong file type: {0}")]
    WrongFileType(String),
}

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Unable to run command")]
    RunError(#[from] RunError),
}

#[cfg(test)]
mod test {
    use crate::{CmdRunner, RunError, shfile::ShFile};
    use async_trait::async_trait;
    use std::{os::unix::process::ExitStatusExt, process::ExitStatus, process::Output};

    #[tokio::test]
    async fn claim_good() {
        let mut r = MockRunner::new();
        r.o.stdout = b"regular file".to_vec();
        let f = ShFile::new(r);
        f.claim("i dont care").await.unwrap();

    }

    #[tokio::test]
    async fn claim_wrong_filetype() {
        let r = MockRunner::new();
        let f = ShFile::new(r);
        if f.claim("i dont care").await.is_ok() { panic!("It should be an error") }
    }

    struct MockRunner {
        o: Output,
    }

    impl MockRunner {
        fn new() -> Self {
            Self {
                o: Output {
                    stdout: b"foo".to_vec(),
                    stderr: b"bar".to_vec(),
                    status: ExitStatus::from_raw(0),
                },
            }
        }
    }

    #[async_trait]
    impl CmdRunner for MockRunner {
        async fn run(&self, _cmd: &str, _args: &[&str]) -> Result<Output, RunError> {
            Ok(self.o.clone())
        }
    }
}
