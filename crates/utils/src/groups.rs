use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString, IntoStaticStr};
use uuid::Uuid;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    rkyv::Deserialize,
    rkyv::Serialize,
    rkyv::Archive,
    AsRefStr,
    IntoStaticStr,
    EnumIs,
    EnumString,
    Display,
    EnumIter,
)]
#[archive(compare(PartialEq), check_bytes)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::prelude::DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
pub enum GroupType {
    #[default]
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Individual"))]
    List,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Group"))]
    Group,
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
pub struct Group {
    pub group_type: GroupType,
    pub name: String,
    pub description: String,
}
impl Group {
    pub fn get_mailbox_id_from_namespace(&self, namespace: &Uuid) -> Uuid {
        Uuid::new_v5(namespace, self.name.as_bytes())
    }
}

#[cfg(test)]
mod group_tests {
    use uuid::Uuid;

    use crate::groups::{Group, GroupType};

    include!("../../../tests/shared_constants.rs");
    #[test]
    pub fn test() {
        let group = Group {
            group_type: GroupType::Group,
            name: TEST_GROUP_NAME.to_string(),
            description: TEST_GROUP_DESCRIPTION.to_string(),
        };
        assert_eq!(
            group.get_mailbox_id_from_namespace(&GROUP_NAMESPACE),
            TEST_GROUP_UUID
        );

        let serialized = rkyv::to_bytes::<_, 256>(&group).unwrap().to_vec();
        let deserialized: Group = rkyv::from_bytes(&serialized).unwrap();

        assert_eq!(deserialized, group);
    }
}
