use std::str::FromStr;

use esbuild_metafile::error::Error;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;

use esbuild_metafile_tests::esbuild_meta_shared_entry_point::ESBUILD_META_SHARED_ENTRY_POINT;

const PARSES_TO_RULE_OUT_HASH_ORDER: usize = 64;

#[test]
fn input_lookup_order_does_not_depend_on_hash_seed() -> Result<(), Error> {
    let first_lookup =
        EsbuildMetafile::from_str(ESBUILD_META_SHARED_ENTRY_POINT)?.input("src/app.ts");

    for _ in 1..PARSES_TO_RULE_OUT_HASH_ORDER {
        let metafile = EsbuildMetafile::from_str(ESBUILD_META_SHARED_ENTRY_POINT)?;

        assert_eq!(metafile.input("src/app.ts"), first_lookup);
    }

    Ok(())
}
