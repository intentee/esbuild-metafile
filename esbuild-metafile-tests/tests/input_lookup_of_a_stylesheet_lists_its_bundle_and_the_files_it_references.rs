use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;

use esbuild_metafile_tests::esbuild_meta_fonts::ESBUILD_META_FONTS;
use esbuild_metafile_tests::paths::paths;

#[test]
fn input_lookup_of_a_stylesheet_lists_its_bundle_and_the_files_it_references() -> Result<(), Error>
{
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_FONTS)?;

    assert_eq!(
        metafile.input("resources/css/page-common.css"),
        InputLookup::Found(InputProperties {
            outputs: paths(&["static/page-common_DO3RNJ3I.css", "static/test_6D5OPEBZ.svg"]),
            preloads: paths(&[
                "https://fonts/font1.woff2",
                "https://fonts/font2.woff2",
                "static/test_6D5OPEBZ.svg"
            ]),
            static_paths: Vec::new(),
        })
    );

    Ok(())
}
