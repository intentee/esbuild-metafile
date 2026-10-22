use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;

use esbuild_metafile_tests::esbuild_meta_dedup::ESBUILD_META_DEDUP;
use esbuild_metafile_tests::paths::paths;

#[test]
fn output_shared_by_bundle_and_import_is_listed_once() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_DEDUP)?;

    assert_eq!(
        metafile.input("src/entry.ts"),
        InputLookup::Found(InputProperties {
            outputs: paths(&["dist/entry.js", "dist/shared.js"]),
            preloads: paths(&["dist/shared.js"]),
            static_paths: Vec::new(),
        })
    );

    Ok(())
}
