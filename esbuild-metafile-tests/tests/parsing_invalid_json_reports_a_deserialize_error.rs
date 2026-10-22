use std::error::Error as _;
use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;

#[test]
fn parsing_invalid_json_reports_a_deserialize_error() {
    let error = EsbuildMetafile::from_str("not valid json").err();

    assert!(matches!(error, Some(Error::Deserialize(_))));
    assert_eq!(
        error.as_ref().map(ToString::to_string),
        Some("failed to deserialize esbuild metafile".to_string())
    );
    assert!(error.as_ref().and_then(|error| error.source()).is_some());
}
