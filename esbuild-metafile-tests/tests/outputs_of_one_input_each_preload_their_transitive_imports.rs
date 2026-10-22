use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;

use esbuild_metafile_tests::esbuild_meta_shared_entry_point::ESBUILD_META_SHARED_ENTRY_POINT;
use esbuild_metafile_tests::paths::paths;

#[test]
fn outputs_of_one_input_each_preload_their_transitive_imports() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_SHARED_ENTRY_POINT)?;

    assert_eq!(
        metafile.output("dist/app.js"),
        OutputLookup::Found(OutputProperties {
            preloads: paths(&["dist/chunk-shared.js", "dist/chunk-deep.js", "dist/chunk-app.js"]),
        })
    );
    assert_eq!(
        metafile.output("dist/app-legacy.js"),
        OutputLookup::Found(OutputProperties {
            preloads: paths(&[
                "dist/chunk-shared.js",
                "dist/chunk-deep.js",
                "dist/chunk-legacy.js"
            ]),
        })
    );

    Ok(())
}
