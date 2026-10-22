use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;
use esbuild_metafile::output_properties::OutputProperties;

use esbuild_metafile_tests::esbuild_meta_fonts::ESBUILD_META_FONTS;
use esbuild_metafile_tests::paths::paths;

#[test]
fn output_lookup_of_a_script_preloads_its_imports_and_its_stylesheet_bundle_imports()
-> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_FONTS)?;

    assert_eq!(
        metafile.output("static/controller_foo_CTJMZK66.js"),
        OutputLookup::Found(OutputProperties {
            preloads: paths(&[
                "https://fonts/font3.woff2",
                "https://fonts/font1.woff2",
                "static/chunk-EMZKCXNJ.js",
                "static/chunk-PI4ZFSEL.js",
                "static/logo_XSTJPNLH.png"
            ]),
        })
    );

    Ok(())
}
