use serde::Serialize;
use tcm::runner::{CmdRunner, LocalCmdRunner, RunError};
use thiserror::Error;

///////////////////////////// MAIN /////////////////////
#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let l = LocalCmdRunner {};
    let f = ShFile::new(l).claim("/etc/hosts").await?;
    let s = toml::to_string(&f)?;
    println!("{s}");

    Ok(())
}

////////////////////////////// ShFile ///////////////////

#[derive(Serialize)]
struct ShFile<T: CmdRunner> {
    #[serde(skip)]
    r: T,
    path: String,
}

impl<T: CmdRunner> ShFile<T> {
    fn new(r: T) -> Self {
        Self {
            path: String::new(),
            r,
        }
    }

    async fn claim(mut self, filename: &str) -> Result<Self, ClaimError> {
        let cmd = &["stat", "--format", "%F", filename];
        let out = self.r.run(cmd).await?.trim().to_string();
        if out.as_str() == "regular file" {
            self.path = filename.to_string();
            Ok(self)
        } else {
            Err(ClaimError::WrongFileType(out))
        }
    }
}

#[derive(Debug, Error)]
enum ClaimError {
    #[error("Unable to run command")]
    RunError(#[from] RunError),
    #[error("Wrong file type: {0}")]
    WrongFileType(String),
}
