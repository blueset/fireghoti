#[derive(Debug, PartialEq)]
#[crate::export(object)]
pub struct Acct {
    pub username: String,
    pub host: Option<String>,
}

#[crate::export]
pub fn string_to_acct(acct: &str) -> Acct {
    let split: Vec<&str> = if let Some(stripped) = acct.strip_prefix('@') {
        stripped
    } else {
        acct
    }
    .split('@')
    .collect();

    Acct {
        username: split[0].to_string(),
        host: if split.len() == 1 {
            None
        } else {
            Some(split[1].to_string())
        },
    }
}

#[crate::export]
pub fn acct_to_string(acct: &Acct) -> String {
    match &acct.host {
        Some(host) => format!("{}@{}", acct.username, host),
        None => acct.username.clone(),
    }
}

#[cfg(test)]
mod unit_test {
    use super::{acct_to_string, string_to_acct, Acct};

    #[test]
    fn test_acct_to_string() {
        let remote_acct = Acct {
            username: "firefish".to_string(),
            host: Some("example.com".to_string()),
        };
        let local_acct = Acct {
            username: "MisakaMikoto".to_string(),
            host: None,
        };

        assert_eq!(acct_to_string(&remote_acct), "firefish@example.com");
        assert_ne!(acct_to_string(&remote_acct), "mastodon@example.com");
        assert_eq!(acct_to_string(&local_acct), "MisakaMikoto");
        assert_ne!(acct_to_string(&local_acct), "ShiraiKuroko");
    }

    #[test]
    fn test_string_to_acct() {
        let remote_acct = Acct {
            username: "firefish".to_string(),
            host: Some("example.com".to_string()),
        };
        let local_acct = Acct {
            username: "MisakaMikoto".to_string(),
            host: None,
        };

        assert_eq!(string_to_acct("@firefish@example.com"), remote_acct);
        assert_eq!(string_to_acct("firefish@example.com"), remote_acct);
        assert_eq!(string_to_acct("@MisakaMikoto"), local_acct);
        assert_eq!(string_to_acct("MisakaMikoto"), local_acct);
    }
}
