use crate::directory_type::Directory;
use utils::service::ServiceAccess;
use uuid::Uuid;

pub mod directory_service;
pub mod directory_type;

pub static SOCKET_NAME: &str = "nitro_mail_directory_service";
#[derive(Debug, Clone, PartialEq, Eq, rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
#[archive(compare(PartialEq), check_bytes)]
pub struct ValidateDirectoryRequest {
    pub group_namespace: Uuid,
    pub account_namespace: Uuid,
}
