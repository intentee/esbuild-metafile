use std::str::FromStr;
use std::sync::Arc;

use crate::esbuild_metafile::EsbuildMetafile;

const ESBUILD_CONTENTS_BASIC: &str = include_str!("./fixtures/esbuild-meta-basic.json");
const ESBUILD_CONTENTS_DEDUP: &str = include_str!("./fixtures/esbuild-meta-dedup.json");
const ESBUILD_CONTENTS_FONTS: &str = include_str!("./fixtures/esbuild-meta-fonts.json");
const ESBUILD_CONTENTS_GLB: &str = include_str!("./fixtures/esbuild-meta-glb.json");
const ESBUILD_CONTENTS_ORPHAN: &str = include_str!("./fixtures/esbuild-meta-orphan.json");
const ESBUILD_CONTENTS_SVG: &str = include_str!("./fixtures/esbuild-meta-svg.json");

pub fn get_metafile_basic() -> Arc<EsbuildMetafile> {
    Arc::new(EsbuildMetafile::from_str(ESBUILD_CONTENTS_BASIC).expect("basic fixture parses"))
}

pub fn get_metafile_dedup() -> Arc<EsbuildMetafile> {
    Arc::new(EsbuildMetafile::from_str(ESBUILD_CONTENTS_DEDUP).expect("dedup fixture parses"))
}

pub fn get_metafile_fonts() -> Arc<EsbuildMetafile> {
    Arc::new(EsbuildMetafile::from_str(ESBUILD_CONTENTS_FONTS).expect("fonts fixture parses"))
}

pub fn get_metafile_glb() -> Arc<EsbuildMetafile> {
    Arc::new(EsbuildMetafile::from_str(ESBUILD_CONTENTS_GLB).expect("glb fixture parses"))
}

pub fn get_metafile_orphan() -> Arc<EsbuildMetafile> {
    Arc::new(EsbuildMetafile::from_str(ESBUILD_CONTENTS_ORPHAN).expect("orphan fixture parses"))
}

pub fn get_metafile_svg() -> Arc<EsbuildMetafile> {
    Arc::new(EsbuildMetafile::from_str(ESBUILD_CONTENTS_SVG).expect("svg fixture parses"))
}
