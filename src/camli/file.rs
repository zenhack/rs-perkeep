use crate::blob;
use super::bytes;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<T> {
    #[serde(flatten)]
    pub shared: Shared,

    #[serde(flatten)]
    pub specific: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Name {
    FileName(String),
    FileNameBytes(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnixProps {
    // TODO: would be better to store as a number and format as octal
    // string, but I(isd) need to figure out how to get serde to do that.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_owner_id: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_group_id: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_group: Option<String>,

    // TODO: times.
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shared {
    #[serde(flatten)]
    name: Name,

    #[serde(flatten)]
    unix_props: UnixProps,
}

impl Shared {
    pub fn from_name(name: Name) -> Self {
        Shared{
            name: name,
            unix_props: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    #[serde(flatten)]
    bytes: bytes::Bytes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    entries: blob::Ref,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Symlink {
    SymlinkTarget(String),
    SymlinkTargetBytes(Vec<MixedEncodingPath>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Socket {}

#[derive(Serialize, Deserialize, Debug)]
pub struct FIFO {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MixedEncodingPath {
    Utf8(String),
    Byte(u8),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Inode {
    pub inode_id: usize,
    pub device_id: usize,
    pub num_links: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Any {
    File(File),
    Directory(Directory),
    Symlink(Symlink),
    Socket(Socket),
    FIFO(FIFO),
}
