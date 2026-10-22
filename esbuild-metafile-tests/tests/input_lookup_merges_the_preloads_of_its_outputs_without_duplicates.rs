use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;

use esbuild_metafile_tests::esbuild_meta_shared_entry_point::ESBUILD_META_SHARED_ENTRY_POINT;
use esbuild_metafile_tests::paths::paths;

#[test]
fn input_lookup_merges_the_preloads_of_its_outputs_without_duplicates() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_SHARED_ENTRY_POINT)?;

    assert_eq!(
        metafile.input("src/app.ts"),
        InputLookup::Found(InputProperties {
            outputs: paths(&[
                "dist/app-legacy.js",
                "dist/chunk-shared.js",
                "dist/chunk-deep.js",
                "dist/chunk-legacy.js",
                "dist/app.js",
                "dist/chunk-app.js"
            ]),
            preloads: paths(&[
                "dist/chunk-shared.js",
                "dist/chunk-deep.js",
                "dist/chunk-legacy.js",
                "dist/chunk-app.js"
            ]),
            static_paths: Vec::new(),
        })
    );

    Ok(())
}
