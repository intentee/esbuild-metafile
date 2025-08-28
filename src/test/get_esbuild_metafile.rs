use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;

use crate::EsbuildMetaFile;

const ESBUILD_CONTENTS_BASIC: &str = include_str!("./fixtures/esbuild-meta-basic.json");
const ESBUILD_CONTENTS_FONTS: &str = include_str!("./fixtures/esbuild-meta-fonts.json");

pub fn get_metafile_basic() -> Result<Arc<EsbuildMetaFile>> {
    Ok(Arc::new(EsbuildMetaFile::from_str(ESBUILD_CONTENTS_BASIC)?))
}

pub fn get_metafile_fonts() -> Result<Arc<EsbuildMetaFile>> {
    Ok(Arc::new(EsbuildMetaFile::from_str(ESBUILD_CONTENTS_FONTS)?))
}
