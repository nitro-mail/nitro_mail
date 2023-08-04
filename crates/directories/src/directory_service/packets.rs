use rkyv::{Archive, Deserialize, Serialize};
use uuid::Uuid;

use helper_macros::ToServicePacket;
use utils::account::Account;
use utils::service_configuration::ServiceConfigurationResponse;

use crate::directory_type::Directory;
use crate::ValidateDirectoryRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Archive, ToServicePacket)]
#[archive(compare(PartialEq), check_bytes)]
#[non_exhaustive]
#[service_packet(
    service_type = Directory,
    from_service_type = FromServiceSystemPackets,
)]
pub enum ToServiceSystemPackets {
    #[packet(
    service_method = Directory::validate_config,
    from_service_variant = FromServiceSystemPackets::ValidateConfigurations
    )]
    ValidateConfigurations(ValidateDirectoryRequest),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Archive, ToServicePacket)]
#[archive(compare(PartialEq), check_bytes)]
#[non_exhaustive]
#[service_packet(
service_type = Directory,
from_service_type = FromServicePackets,
)]
pub enum ToServicePackets {
    #[packet(
    sub_packet = ToServiceSystemPackets,
    from_service_variant = FromServicePackets::SystemPacket
    )]
    SystemPacket(ToServiceSystemPackets),
    #[packet(
    service_method = Directory::get_account,
    from_service_variant = FromServicePackets::GetAccount
    )]
    GetAccount(String),
    #[packet(
    service_method = Directory::login_account,
    from_service_variant = FromServicePackets::LoginAccount
    )]
    LoginAccount { username: String, password: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Archive)]
#[archive(compare(PartialEq), check_bytes)]
#[non_exhaustive]
pub enum FromServiceSystemPackets {
    ValidateConfigurations(ServiceConfigurationResponse),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Archive)]
#[archive(compare(PartialEq), check_bytes)]
#[non_exhaustive]
pub enum FromServicePackets {
    SystemPacket(FromServiceSystemPackets),
    GetAccount(Option<Account>),
    LoginAccount(Option<Account>),
    /// If the account is valid then valid is true
    /// If the account is invalid then valid is false
    ///
    /// Account is present if it is valid but changes are made to the account
    ///
    ValidateAccount {
        account: Option<Account>,
        valid: bool,
    },
    InternalDirectoryError(String),
    NoResponse,
}
