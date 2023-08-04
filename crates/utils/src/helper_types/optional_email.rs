use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use serde::Serialize;

use crate::helper_types::EmailAddress;

/// A newtype wrapper around an [Option] with type of [EmailAddress](super::email_address::EmailAddress)
///
/// That upon Deserialization will count an empty string as [Option::None]
///
/// ## Notes
/// If any characters are inside the String upon Deserialization it will assume you are trying to give an Email and will result in an Invalid Email Error
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct OptionalEmailAddress(pub Option<EmailAddress>);

impl OptionalEmailAddress {
    pub fn into_inner(self) -> Option<EmailAddress> {
        self.0
    }
}
impl Deref for OptionalEmailAddress {
    type Target = Option<EmailAddress>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<Option<EmailAddress>> for OptionalEmailAddress {
    fn as_ref(&self) -> &Option<EmailAddress> {
        &self.0
    }
}

impl From<Option<EmailAddress>> for OptionalEmailAddress {
    fn from(email_address: Option<EmailAddress>) -> Self {
        Self(email_address)
    }
}
impl From<EmailAddress> for OptionalEmailAddress {
    fn from(email_address: EmailAddress) -> Self {
        Self(Some(email_address))
    }
}

mod _serde {
    use crate::helper_types::{EmailAddress, OptionalEmailAddress};

    impl<'de> serde::Deserialize<'de> for OptionalEmailAddress {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = Option::<String>::deserialize(deserializer)?;
            if let Some(s) = s {
                if s.is_empty() {
                    Ok(OptionalEmailAddress(None))
                } else {
                    Ok(OptionalEmailAddress(Some(
                        EmailAddress::new(s).map_err(serde::de::Error::custom)?,
                    )))
                }
            } else {
                Ok(OptionalEmailAddress(None))
            }
        }
    }
}
