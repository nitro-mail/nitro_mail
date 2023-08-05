use crate::configs::{Config, ConfigName};
use ahash::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DomainConfiguration {
    pub domains: HashMap<String, Domain>,
}
impl Config for DomainConfiguration {
    fn config_header() -> Option<&'static str>
    where
        Self: Sized,
    {
        // TODO Add link to docs
        Some("Config https://docs.nitro-mail.dev/configs/domains")
    }

    fn config_name() -> ConfigName
    where
        Self: Sized,
    {
        ConfigName::Name("domains.toml")
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Domain {
    pub domain: String,
    pub sign_with: Vec<String>,
}
