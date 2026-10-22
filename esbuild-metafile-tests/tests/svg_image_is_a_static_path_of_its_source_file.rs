use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;

use esbuild_metafile_tests::esbuild_meta_svg::ESBUILD_META_SVG;
use esbuild_metafile_tests::paths::paths;

#[test]
fn svg_image_is_a_static_path_of_its_source_file() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_SVG)?;

    assert_eq!(
        metafile.input("resources/images/image.svg"),
        InputLookup::Found(InputProperties {
            outputs: paths(&["dist/image_123.js", "dist/image_123.svg"]),
            preloads: paths(&["dist/image_123.svg"]),
            static_paths: paths(&["dist/image_123.svg"]),
        })
    );

    Ok(())
}
