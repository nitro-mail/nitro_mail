use crate::configs::{Config, ConfigDuration, ConfigName, PathOrType};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::PathBuf;
use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString, IntoStaticStr};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    AsRefStr,
    IntoStaticStr,
    EnumIs,
    EnumString,
    Display,
    EnumIter,
)]
pub enum DmarcSigningAlgorithm {
    #[strum(serialize = "rsa-sha256")]
    #[serde(rename = "rsa-sha256")]
    RsaSha256,
    #[strum(serialize = "rsa-sha1")]
    #[serde(rename = "rsa-sha1")]
    RsaSha1,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    AsRefStr,
    IntoStaticStr,
    EnumIs,
    EnumString,
    Display,
    EnumIter,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CanonicalizationMethod {
    Simple,
    Relaxed,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canonicalization {
    pub header: CanonicalizationMethod,
    pub body: CanonicalizationMethod,
}
impl Display for Canonicalization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.header, self.body)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    AsRefStr,
    IntoStaticStr,
    EnumIs,
    EnumString,
    Display,
    EnumIter,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ThirdPartyAlgorithm {
    SHA1,
    SHA256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSignature {
    /// The DKIM Key Name
    pub dkim_name: String,
    /// The path to the private key
    pub private_key: PathBuf,
    /// The domain this is signed for
    pub domain: String,
    /// The selector for the DKIM record
    pub selector: String,
    /// Headers to Sign For
    pub headers: Vec<String>,
    /// The algorithm to use for signing
    pub algorithm: DmarcSigningAlgorithm,
    /// The canonicalization method to use for signing
    pub canonicalization: Canonicalization,
    pub expiration: Option<ConfigDuration>,
    // TODO pub third_party_rules: ThirdPartyRules,
    // TODO pub auid: Option<String>,
    pub set_body_length: Option<bool>,
    pub report: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyRules {
    pub third_party: String,
    pub third_party_algo: ThirdPartyAlgorithm,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DKIMConfig {
    pub dkim_signatures: Vec<PathOrType<DomainSignature>>,
    // TODO DKIN Report Config
}

impl Default for DKIMConfig {
    fn default() -> Self {
        Self {
            dkim_signatures: vec![],
        }
    }
}

impl Config for DKIMConfig {
    fn config_header() -> Option<&'static str>
    where
        Self: Sized,
    {
        Some("Please Visit https://docs.nitro-mail.dev/configs/dkim")
    }

    fn config_name() -> ConfigName
    where
        Self: Sized,
    {
        ConfigName::Name("dkim.toml")
    }
}
