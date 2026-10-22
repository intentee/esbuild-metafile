use std::collections::HashSet;
use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use esbuild_metafile::input_lookup::InputLookup;

use esbuild_metafile_tests::esbuild_meta_orphan::ESBUILD_META_ORPHAN;
use esbuild_metafile_tests::paths::paths;

#[test]
fn orphan_output_is_listed_but_belongs_to_no_input() -> Result<(), Error> {
    let metafile = EsbuildMetafile::from_str(ESBUILD_META_ORPHAN)?;

    assert_eq!(
        metafile.get_output_paths(),
        paths(&["dist/orphan.js"])
            .into_iter()
            .collect::<HashSet<String>>()
    );
    assert_eq!(metafile.input("dist/orphan.js"), InputLookup::NotFound);

    Ok(())
}
