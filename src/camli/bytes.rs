use crate::blob;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PartLoc {
    BlobRef(blob::Ref),
    BytesRef(blob::Ref),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bytes {
    parts: Vec<Part>,
}

impl Bytes {
    pub fn new(parts: Vec<Part>) -> Self {
        Bytes{
            parts: parts,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub size: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub loc: Option<PartLoc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}
