use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::output_lookup::OutputLookup;

use esbuild_metafile_tests::esbuild_meta_basic::ESBUILD_META_BASIC;

#[test]
fn output_lookup_of_an_unknown_output_is_not_found() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_BASIC)?;

    assert_eq!(
        metafile.output("dist/does-not-exist.js"),
        OutputLookup::NotFound
    );

    Ok(())
}
