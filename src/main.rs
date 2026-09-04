use serde::Serialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

fn main() -> color_eyre::eyre::Result<()> {
    let f = FileName::from("/etc/hosts".to_string());

    let ff = File::try_claim(f)?;
    let s = toml::to_string(&ff)?;

    println!("{s}");

    Ok(())
}

#[derive(Serialize)]
struct FileName {
    path: PathBuf,
}

#[derive(Serialize)]
struct File {
    name: FileName,
}

impl From<String> for FileName {
    fn from(s: String) -> Self {
        Self { path: s.into() }
    }
}

impl File {
    fn try_claim(f: FileName) -> Result<Self, ClaimError> {
        match fs::metadata(&f.path) {
            Err(e) => Err(ClaimError(format!("{}: {}", f.path.display(), e))),
            Ok(_) => Ok(Self { name: f})
        }
    }
}

#[derive(Debug, Error)]
#[error("Unable to claim {0}")]
struct ClaimError(String);
