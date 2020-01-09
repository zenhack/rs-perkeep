mod blob;
mod camli;

fn print(b: camli::Common) {
    let serialized = serde_json::to_string(&b).unwrap();
    println!("{}", serialized);
}

fn main() {
    use camli::*;
    use camli::bytes::*;
    print(Common::new(Any::Bytes(Bytes::new(vec![
        Part {
            size: 0,
            loc: Some(PartLoc::BlobRef(blob::Ref::from_str("Hello, World!"))),
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
