use rkyv::{Archive, Deserialize, Serialize};
use uuid::Uuid;

use crate::common_types::{AccountType, EmailType};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Archive,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(compare(PartialEq), check_bytes)]
pub struct Account {
    pub username: String,
    pub account_type: AccountType,
}
impl Account {
    pub fn get_mailbox_id_from_namespace(&self, namespace: &Uuid) -> Uuid {
        Uuid::new_v5(namespace, self.username.as_bytes())
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(compare(PartialEq), check_bytes)]
pub struct EmailAddress {
    pub email_address: String,
    pub email_type: EmailType,
    pub mailbox_id: Uuid,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::account::{Account, EmailAddress};
    use crate::common_types::EmailType;

    #[test]
    pub fn test_serialize_and_deserialize() {
        let account = Account {
            username: "test".to_string(),
            account_type: Default::default(),
        };

        let serialized = rkyv::to_bytes::<_, 256>(&account).unwrap().to_vec();
        let deserialized: Account = rkyv::from_bytes(&serialized).unwrap();
        assert_eq!(account, deserialized)
    }
    #[test]
    pub fn test_serialize_and_deserialize_email() {
        let account = super::EmailAddress {
            email_address: "test@localhost".to_string(),
            email_type: EmailType::Primary,
            mailbox_id: Uuid::new_v4(),
        };

        let serialized = rkyv::to_bytes::<_, 256>(&account).unwrap().to_vec();
        let deserialized: EmailAddress = rkyv::from_bytes(&serialized).unwrap();
        assert_eq!(account, deserialized)
    }
}
