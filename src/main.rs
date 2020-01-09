pub mod blob {
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Ref(String);

    impl Ref {
        pub fn from_string(s: String) -> Self {
            Ref(s)
        }

        pub fn from_str(s: &str) -> Self {
            Ref::from_string(s.to_string())
        }
    }
}

pub mod camli {
    use serde::{Serialize, Deserialize};

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
        Bytes(bytes::Bytes),

        #[serde(rename = "static-set")]
        StaticSet(StaticSet),

        File(file::Common<file::File>),
        Directory(file::Common<file::Directory>),
        Symlink(file::Common::<file::Symlink>),
        Socket(file::Common::<file::Socket>),
        FIFO(file::Common::<file::FIFO>),
    }

    pub mod file {
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
            shared: Shared,
        }

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Directory {
            #[serde(flatten)]
            shared: Shared,

            entries: crate::blob::Ref,
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
        pub enum Any {
            File(File),
            Directory(Directory),
            Symlink(Symlink),
            Socket(Socket),
            FIFO(FIFO),
        }
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct StaticSet {
        pub members: Vec<crate::blob::Ref>,
        pub merge_sets: Vec<crate::blob::Ref>,
    }

    pub mod bytes {
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub enum PartLoc {
            BlobRef(crate::blob::Ref),
            BytesRef(crate::blob::Ref),
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
    }
}

fn print(blob: camli::Common) {
    let serialized = serde_json::to_string(&blob).unwrap();
    println!("{}", serialized);
}

fn main() {
    use camli::*;
    use camli::bytes::*;
    print(Common::new(Any::Bytes(Bytes::new(vec![
        Part {
            size: 0,
            loc: Some(PartLoc::BlobRef(crate::blob::Ref::from_str("Hello, World!"))),
            offset: Some(1),
        },
        Part {
            size: 12,
            loc: None,
            offset: None,
        },
    ]))));
    print(Common::new(Any::StaticSet(StaticSet{
        members: vec![],
        merge_sets: vec![],
    })));
    print(Common::new(Any::Symlink(file::Common{
        shared: file::Shared::from_name(file::Name::FileName("thelink".to_string())),
        specific: file::Symlink::SymlinkTarget("thefile".to_string()),
    })));
    print(Common::new(Any::Symlink(file::Common{
        shared: file::Shared::from_name(file::Name::FileName("thelink".to_string())),
        specific: file::Symlink::SymlinkTargetBytes(
            vec![file::MixedEncodingPath::Utf8("thefile".to_string())],
        ),
    })))
}
