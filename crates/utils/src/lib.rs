use std::io::{Read, Write};
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod account;
pub mod chrono_serde;
pub mod common_types;
pub mod configs;
pub mod groups;
pub mod helper_types;
pub mod interprocess_guard;
pub mod service;
pub mod service_configuration;
