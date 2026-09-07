use crate::localrunner::{CmdRunner, RunError};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ShFile<T: CmdRunner> {
    #[serde(skip)]
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
        let cmd = &["stat", "--format", "%F", filename];
        let out = self.r.run(cmd).await?.trim().to_string();
        if out.as_str() == "regular file" {
            self.path = filename.to_string();
            Ok(self)
        } else {
            Err(ClaimError::WrongFileType(out))
        }
    }

    pub async fn get_content(&self) -> Result<String, ContentError> {
        let cmd = &["cat", self.path.as_str()];
        let out = self.r.run(cmd).await?.trim().to_string();
        Ok(out)
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
