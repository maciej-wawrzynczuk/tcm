use std::collections::HashMap;

#[derive(Debug)]
pub struct SSHHost {
    pub addr: String,
    pub port: u16,
    pub user: Option<String>,
}

impl SSHHost {
    pub fn from_kv(
        mut kv_entry: HashMap<String, String>,
        hostname_field: &str,
        port_key: Option<&str>,
        user_key: Option<&str>,
    ) -> Result<Self, SSHHostError> {
        let addr = kv_entry
            .remove(hostname_field)
            .ok_or(SSHHostError::NoHostname)?;

        let port = match port_key {
            None => 22,
            Some(k) => kv_entry
                .remove(k)
                .ok_or(SSHHostError::NoPort)?
                .parse::<u16>()?
        };

        let user = match user_key {
            None => None,
            Some(k) => Some(kv_entry
                .remove(k)
                .ok_or(SSHHostError::NoUser)?)
        };

        Ok(Self {
            addr,
            port,
            user,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SSHHostError {
    #[error("no hostname field")]
    NoHostname,
    #[error("no port field")]
    NoPort,
    #[error("wrong number")]
    IntParse(#[from] std::num::ParseIntError),
    #[error("No user")]
    NoUser,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{assert_matches, collections::HashMap};

    #[test]
    fn hostname_is_mandatory() {
        let kv: HashMap<String, String> = HashMap::new();
        let fut = SSHHost::from_kv(kv, "i dont care", None, None);
        assert!(matches!(fut, Err(SSHHostError::NoHostname)));
    }

    #[test]
    fn hostname_only() {
        let host_key = "foo";
        let host_name = "bar";
        let kv = HashMap::from([(host_key.to_string(), host_name.to_string())]);
        let sut = SSHHost::from_kv(kv, host_key, None, None).unwrap();
        assert_eq!(sut.addr, host_name);
    }

    #[test]
    fn port_default() {
        let host_key = "foo";
        let kv = HashMap::from([(host_key.to_string(), "ignore".to_string())]);
        let sut = SSHHost::from_kv(kv, host_key, None, None).unwrap();
        assert_eq!(sut.port, 22);
    }

    #[test]
    fn port_missing_key() {
        let host_field = "h";
        let kv = HashMap::from([
            (host_field.to_string(), "host_name".to_string()),
            ("port".to_string(), "something".to_string()),
        ]);
        let sut = SSHHost::from_kv(kv, host_field, Some("something else"), None);
        assert!(matches!(sut, Err(SSHHostError::NoPort)));
    }

    #[test]
    fn port_good() {
        let port = 1234;
        let port_str = format!("{port}");
        let port_field = "port";
        let hostname_field = "hhhhh";
        let kv = HashMap::from([
            (hostname_field.to_string(), "h".to_string()),
            (port_field.to_string(), port_str),
        ]);
        let sut = SSHHost::from_kv(kv, hostname_field, Some(port_field), None).unwrap();
        assert_eq!(sut.port, port);
    }

    #[test]
    fn port_num_parse() {
        let bad_port = "foo";
        let port_field = "p";
        let host_field = "h";
        let kv_entry = HashMap::from([
            (host_field.to_string(), "who cares".to_string()),
            (port_field.to_string(), bad_port.to_string()),
        ]);
        let sut = SSHHost::from_kv(kv_entry, host_field, Some(port_field), None);
        assert_matches!(sut, Err(SSHHostError::IntParse(_)));
    }

    #[test]
    fn no_user() {
        let host_key = "foo";
        let kv = HashMap::from([(host_key.to_string(), "ignore".to_string())]);
        let sut = SSHHost::from_kv(kv, host_key, None, None).unwrap();
        assert!(sut.user.is_none());
    }

    #[test]
    fn user() {
        let host_key = "foo";
        let user_field = "user";
        let user_name = "my user";
        let kv = HashMap::from([
            (host_key.to_string(), "ignore".to_string()),
            (user_field.to_string(), user_name.to_string())
        ]);
        let sut = SSHHost::from_kv(kv, host_key, None, Some(user_field)).unwrap();
        assert_eq!(sut.user, Some(user_name.to_string()));
    }
}
