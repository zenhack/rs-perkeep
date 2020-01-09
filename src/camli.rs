use crate::blob;
use serde::{Serialize, Deserialize};

pub mod bytes;
pub mod file;
pub mod permanode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Common {
    camli_version: u32,

    #[serde(flatten)]
    blob: Any,
}

impl Common {
    pub fn new(b: Any) -> Self {
        Self {
            camli_version: 1,
            blob: b,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "camliType")]
pub enum Any {
    Permanode(permanode::Permanode),
    Bytes(bytes::Bytes),

    #[serde(rename = "static-set")]
    StaticSet(StaticSet),

    // TODO: ideally we wouldn't flatten these into
    // Any, but instead use file::Any with one enum variant. To do that
    // though, I(isd) need to work out how to get serde to do the tagging
    // right.
    File(file::Common<file::File>),
    Directory(file::Common<file::Directory>),
    Symlink(file::Common::<file::Symlink>),
    Socket(file::Common::<file::Socket>),
    FIFO(file::Common::<file::FIFO>),

    Inode(file::Inode),
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StaticSet {
    pub members: Vec<blob::Ref>,
    pub merge_sets: Vec<blob::Ref>,
}
