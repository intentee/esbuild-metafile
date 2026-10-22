use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;

use esbuild_metafile_tests::esbuild_meta_glb::ESBUILD_META_GLB;
use esbuild_metafile_tests::paths::paths;

#[test]
fn output_lookup_preloads_a_file_imported_by_two_of_its_chunks_once() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_GLB)?;

    assert_eq!(
        metafile.output("dist/main.js"),
        OutputLookup::Found(OutputProperties {
            preloads: paths(&["dist/chunk-ABC.js", "dist/model_123.glb", "dist/chunk-DEF.js"]),
        })
    );

    Ok(())
}
