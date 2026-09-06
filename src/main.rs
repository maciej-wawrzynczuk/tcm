use serde::Serialize;
use tokio::process::Command;
use thiserror::Error;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let l = LocalCmdRunner {};
    let f = ShFile::new(l)
        .claim("/etc/hosts").await?;
    let s = toml::to_string(&f)?;
    println!("{s}");

    Ok(())
}

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
        let cmd = &[
            "stat",
            "--format",
            "%F",
            filename
        ];
        let out = self.r.run(cmd).await?.trim().to_string();
        if out.as_str() == "regular file" {
            self.path = filename.to_string();
            Ok(self)
        } else {
            Err(ClaimError::WrongFileType(out))
        }

    }
}

// Do I need stderr in normal situations?
trait CmdRunner {
    async fn run(&self, cmd: &[&str]) -> Result<String, RunError>;
}

struct LocalCmdRunner {}

impl CmdRunner for LocalCmdRunner {
    async fn run(&self, cmd: &[&str]) -> Result<String, RunError> {
        let cmd0 = cmd.first().ok_or(RunError::NoCommandProvided)?;
        let args = &cmd[1..];
        let o = Command::new(cmd0).args(args).output().await?;
        if o.status.success() {
            Ok(String::from_utf8_lossy(&o.stdout).into_owned())
        } else {
            Err(RunError::CommandFailed {
                code: o.status.code().unwrap_or(-1), // None if process terminated by signal. TODO:
                // Handle it better
                stderr: String::from_utf8_lossy(&o.stderr).into_owned(),
            })
        }
    }
}

#[derive(Debug, Error)]
enum ClaimError {
    #[error("Unable to run command")]
    RunError(#[from] RunError),
    #[error("Wrong file type: {0}")]
    WrongFileType(String)
} 

#[derive(Debug, Error)]
enum RunError {
    #[error("process spawn error {0}")]
    SpawnFailed(#[from] std::io::Error),
    #[error("no command provided")]
    NoCommandProvided,
    #[error("command failed witch {code}, {stderr}")]
    CommandFailed { code: i32, stderr: String },
}
