/// The unique identifier for a file on Hedera.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct FileId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}
