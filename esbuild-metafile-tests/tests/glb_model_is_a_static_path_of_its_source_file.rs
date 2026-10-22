use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;

use esbuild_metafile_tests::esbuild_meta_glb::ESBUILD_META_GLB;
use esbuild_metafile_tests::paths::paths;

#[test]
fn glb_model_is_a_static_path_of_its_source_file() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_GLB)?;

    assert_eq!(
        metafile.input("resources/media/models/model.glb"),
        InputLookup::Found(InputProperties {
            outputs: Vec::new(),
            preloads: Vec::new(),
            static_paths: paths(&["dist/model_123.glb"]),
        })
    );

    Ok(())
}
