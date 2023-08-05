#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
#[archive(compare(PartialEq), check_bytes)]
pub struct GitInfo {
    pub commit: String,
    pub branch: String,
    pub commit_date: String,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
#[archive(compare(PartialEq), check_bytes)]
pub enum ServiceType {
    Directory,
    Storage,
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
pub enum ServiceConfigurationResponse {
    /// If you ever see this error.
    /// It means the Namespaces of the internal service or nitro_mail has been changed.
    /// This is a critical error. Could mean a loss of data.
    NamespaceMismatch,

    Success {
        new_install: bool,
        internal_service_name: String,
        git: GitInfo,
        service_type: ServiceType,
        version: String,
    },
}
