use std::collections::HashSet;
use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;

use esbuild_metafile_tests::esbuild_meta_basic::ESBUILD_META_BASIC;
use esbuild_metafile_tests::paths::paths;

#[test]
fn output_paths_list_every_output_of_the_metafile() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_BASIC)?;

    assert_eq!(
        metafile.get_output_paths(),
        paths(&["dist/main.css", "dist/main.js"])
            .into_iter()
            .collect::<HashSet<String>>()
    );

    Ok(())
}
