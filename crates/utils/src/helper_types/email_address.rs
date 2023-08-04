use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("Invalid Email Address")]
pub struct InvalidEmailAddress;

/// A newtype wrapper around a String that represents an email address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct EmailAddress(String);
impl PartialEq<String> for EmailAddress {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}
impl PartialEq<&str> for EmailAddress {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
impl EmailAddress {
    pub fn new(email_address: impl Into<String>) -> Result<Self, InvalidEmailAddress> {
        let email_address: String = email_address.into();
        let email_address_split = email_address.splitn(2, '@').collect::<Vec<_>>();
        if email_address_split.len() != 2 {
            return Err(InvalidEmailAddress);
        }
        let user = email_address_split[0];
        let domain = email_address_split[1];
        if Self::validate_domain(domain) && Self::validate_user(user) {
            Ok(EmailAddress(email_address))
        } else {
            return Err(InvalidEmailAddress);
        }
    }
    fn validate_domain(domain: &str) -> bool {
        if domain.is_empty() || domain.len() > 255 || !domain.contains(".") {
            false
        } else {
            true
        }
    }
    fn validate_user(user: &str) -> bool {
        if user.is_empty() || user.len() > 64 {
            false
        } else {
            true
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::helper_types::EmailAddress;

    #[test]
    pub fn test_email_address() {
        assert!(EmailAddress::new("test@gmail.com").is_ok());
        assert!(EmailAddress::new("fail.com").is_err());
    }
}
mod _serde {
    use crate::helper_types::EmailAddress;

    impl<'de> serde::Deserialize<'de> for EmailAddress {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            EmailAddress::new(s).map_err(serde::de::Error::custom)
        }
    }
}
impl FromStr for EmailAddress {
    type Err = InvalidEmailAddress;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EmailAddress::new(s)
    }
}

impl TryFrom<String> for EmailAddress {
    type Error = InvalidEmailAddress;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        EmailAddress::new(value)
    }
}
impl Into<String> for EmailAddress {
    fn into(self) -> String {
        self.0
    }
}

impl Deref for EmailAddress {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl AsRef<String> for EmailAddress {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
#[cfg(feature = "sea-orm")]
mod database {
    use sea_orm::sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr};
    use sea_orm::{ColIdx, ColumnType, QueryResult, TryGetError, TryGetable, Value};

    use crate::helper_types::email_address::EmailAddress;

    impl From<EmailAddress> for Value {
        fn from(value: EmailAddress) -> Self {
            Value::String(Some(value.0.into()))
        }
    }
    impl TryGetable for EmailAddress {
        fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
            let value = String::try_get_by(res, index)?;
            Ok(EmailAddress(value))
        }
    }
    impl ValueType for EmailAddress {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::String(Some(s)) => {
                    let email_address: String = *s;
                    Ok(EmailAddress(email_address))
                }
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(EmailAddress).to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::String
        }

        fn column_type() -> ColumnType {
            ColumnType::String(None)
        }
    }

    impl Nullable for EmailAddress {
        fn null() -> Value {
            Value::String(None)
        }
    }
}
