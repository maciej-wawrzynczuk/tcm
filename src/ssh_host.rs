use std::collections::HashMap;

pub struct SSHHost {
    pub addr: String,
    pub port: u16,
    pub user: String,
}

impl SSHHost {
    pub fn from_kv(
        mut kv_entry: HashMap<String, String>,
        hostnae_field: &str,
    ) -> Result<Self, SSHHostError> {
        let addr = kv_entry.remove(hostnae_field).ok_or(SSHHostError::NoHostname)?;
        Ok(Self{
            addr,
            port: 42,
            user: "not yet".to_string()
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SSHHostError {
    #[error("no hostname field")]
    NoHostname,
}


#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;


    #[test]
    fn hotname_is_mandatory() {
        let kv: HashMap<String, String> = HashMap::new();
        let fut = SSHHost::from_kv(kv, "i dont care");
        assert!(matches!(fut, Err(SSHHostError::NoHostname)));
    }
}
