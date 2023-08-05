use serde::{Deserialize, Serialize};
use utils::configs::{Config, ConfigName};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SMTPConfig {
    pub hosts: Vec<SMTPHost>,
}
impl Default for SMTPConfig {
    fn default() -> Self {
        let mut hosts = Vec::new();
        hosts.push(SMTPHost {
            bind: "0.0.0.0:25".to_string(),
            greeting: None,
        });
        hosts.push(SMTPHost {
            bind: "0.0.0.0:587".to_string(),
            greeting: None,
        });
        hosts.push(SMTPHost {
            bind: "0.0.0.0:465".to_string(),
            greeting: None,
        });
        return SMTPConfig { hosts };
    }
}
impl Config for SMTPConfig {
    fn config_header() -> Option<&'static str>
    where
        Self: Sized,
    {
        Some("https://docs.nitro_mail.kingtux.dev/configs/smtp")
    }

    fn config_name() -> ConfigName
    where
        Self: Sized,
    {
        ConfigName::Name("smtp.toml")
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SMTPHost {
    pub bind: String,
    pub greeting: Option<String>,
}
