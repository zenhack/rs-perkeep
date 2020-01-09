pub mod blob;
pub mod camli;

#[cfg(test)]
mod tests {
    use serde_json;
    use super::blob;
    use super::camli;
    use super::camli::*;
    use super::camli::bytes::*;

    fn expect_result(value: camli::Common, expected: &str) {
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized.as_str(), expected);
    }

    #[test]
    fn the_test() {

        expect_result(
            Common::new(Any::StaticSet(StaticSet{
                members: vec![],
                merge_sets: vec![],
            })),
            r#"{"camliVersion":1,"camliType":"static-set","members":[],"mergeSets":[]}"#,
        );
        expect_result(
            Common::new(Any::Bytes(Bytes::new(vec![
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
            ]))),
            r#"{"camliVersion":1,"camliType":"bytes","parts":[{"size":0,"blobRef":"Hello, World!","offset":1},{"size":12}]}"#,
        );
        expect_result(
            Common::new(Any::Symlink(file::Common{
                shared: file::Shared::from_name(file::Name::FileName("thelink".to_string())),
                specific: file::Symlink::SymlinkTarget("thefile".to_string()),
            })),
            r#"{"camliVersion":1,"camliType":"symlink","fileName":"thelink","symlinkTarget":"thefile"}"#,
        );
        expect_result(
            Common::new(Any::Symlink(file::Common{
                shared: file::Shared::from_name(file::Name::FileName("thelink".to_string())),
                specific: file::Symlink::SymlinkTargetBytes(
                    vec![file::MixedEncodingPath::Utf8("thefile".to_string())],
                ),
            })),
            r#"{"camliVersion":1,"camliType":"symlink","fileName":"thelink","symlinkTargetBytes":["thefile"]}"#,
        );
    }
}
