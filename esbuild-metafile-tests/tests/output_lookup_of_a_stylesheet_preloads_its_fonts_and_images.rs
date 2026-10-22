use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;

use esbuild_metafile_tests::esbuild_meta_fonts::ESBUILD_META_FONTS;
use esbuild_metafile_tests::paths::paths;

#[test]
fn output_lookup_of_a_stylesheet_preloads_its_fonts_and_images() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_FONTS)?;

    assert_eq!(
        metafile.output("static/page-common_DO3RNJ3I.css"),
        OutputLookup::Found(OutputProperties {
            preloads: paths(&[
                "https://fonts/font1.woff2",
                "https://fonts/font2.woff2",
                "static/test_6D5OPEBZ.svg"
            ]),
        })
    );

    Ok(())
}
