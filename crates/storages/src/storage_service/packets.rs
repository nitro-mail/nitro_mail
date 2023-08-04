use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Archive)]
#[archive(compare(PartialEq), check_bytes)]
pub enum ToServicePackets {
    TODO,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Archive)]
#[archive(compare(PartialEq), check_bytes)]
pub enum FromServicePackets {
    TODO,
}
