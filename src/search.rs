use serde::{Serialize, Deserialize};
use crate::blob;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    #[serde(flatten)]
    query: Query,

    limit: Option<usize>,
    around: Option<blob::Ref>,

    sort: Option<SortType>,

    #[serde(rename = "continue")]
    continue_: Option<String>,

    // TODO: describe.
}

// FIXME: this should serialize as an int.
#[derive(Serialize, Deserialize, Debug)]
pub enum SortType {
    Unsorted,
    LastModifiedDesc,
    LastModifiedAsc,
    CreatedDesc,
    CreatedAsc,
    BlobRefAsc,
    MapSort,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Query {
    expression(String),
    constriant(Constraint),
}

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camliCase")]
    pub enum Constraint {
        // TODO: are any of these non-mutually-exclusive?
        Logical(constraint::Logical),
        Anything, // TODO: make this a bool?
        CamliType(String),
        BlobRefPrefix(String),
        File(constraint::File),
        Dir(constraint::Dir),
        Claim(constraint::Claim),
        BlobSize(constraint::Int),
        Permanode(constraint::Permanode),
    }

pub mod constraint {

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camliCase")]
    pub struct Logical {
        Not {
            a: Box<Constraint>,
        },
        And {
            a: Box<Constraint>,
            b: Box<Constraint>,
        },
        Or {
            a: Box<Constraint>,
            b: Box<Constraint>,
        },
        Xor {
            a: Box<Constraint>,
            b: Box<Constraint>,
        },
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    #[serde(rename_all = "camliCase")]
    pub struct File {
        file_size: Option<Int>,
        file_name: Option<super::constraint::String>,
        mime_type: Option<super::constraint::String>,
        time: Option<Time>,
        mod_time: Option<Time>,
        whole_ref: Option<blob::Ref>,
        parent_dir: Option<Dir>,

        // for images
        is_image: bool, // optional?

        exif: Option<EXIF>,
        width: Option<Int>,
        height: Option<Int>,
        #[serde(rename = "widthHeightRation")]
        wh_ratio: Option<Float>,
        location: Option<Location>,

        // ID3 and similar media tags
        media_tag: Option<MediaTag>,
    }
}
