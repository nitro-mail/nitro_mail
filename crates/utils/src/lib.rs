use std::io::Write;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod account;
pub mod chrono_serde;
pub mod common_types;
pub mod groups;
pub mod helper_types;
pub mod interprocess_guard;
pub mod service;
pub mod service_configuration;

pub trait Config: DeserializeOwned + Default + Serialize {
    fn config_header() -> &'static str
    where
        Self: Sized;

    fn config_name() -> &'static str
    where
        Self: Sized;

    fn write(&self, target: &mut impl Write) -> Result<(), std::io::Error>
    where
        Self: Sized,
    {
        let config = toml::to_string_pretty(self).unwrap();

        target.write_all("# ".as_bytes())?;
        target.write_all(Self::config_header().as_bytes())?;
        target.write_all("\n".as_bytes())?;
        target.write_all(config.as_bytes())
    }
}
impl Config for () {
    fn config_header() -> &'static str
    where
        Self: Sized,
    {
        "Empty config"
    }

    fn config_name() -> &'static str
    where
        Self: Sized,
    {
        "empty.toml"
    }
}
