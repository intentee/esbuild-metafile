use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;
use esbuild_metafile::input_properties::InputProperties;

use esbuild_metafile_tests::esbuild_meta_fonts::ESBUILD_META_FONTS;
use esbuild_metafile_tests::paths::paths;

#[test]
fn input_lookup_of_a_script_lists_its_bundle_and_its_stylesheet_bundle() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_FONTS)?;

    assert_eq!(
        metafile.input("resources/ts/controller_foo.tsx"),
        InputLookup::Found(InputProperties {
            outputs: paths(&[
                "static/controller_foo_CTJMZK66.js",
                "static/controller_foo_CX2Z63ZH.css"
            ]),
            preloads: paths(&[
                "https://fonts/font3.woff2",
                "https://fonts/font1.woff2",
                "static/chunk-EMZKCXNJ.js",
                "static/chunk-PI4ZFSEL.js",
                "static/logo_XSTJPNLH.png"
            ]),
            static_paths: Vec::new(),
        })
    );

    Ok(())
}
