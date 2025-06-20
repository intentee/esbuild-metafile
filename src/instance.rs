use std::str::FromStr;
use std::sync::Arc;

use once_cell::sync::OnceCell;

use super::EsbuildMetaFile;

pub static INSTANCE: OnceCell<Arc<EsbuildMetaFile>> = OnceCell::new();

pub fn initialize_instance(esbuild_meta_contents: &str) {
    let esbuild_metafile = Arc::new(
        EsbuildMetaFile::from_str(esbuild_meta_contents).expect("Failed to parse ESBuild metafile"),
    );

    INSTANCE
        .set(esbuild_metafile)
        .expect("Failed to set ESBuild metafile instance");
}

pub fn get_esbuild_metafile() -> Arc<EsbuildMetaFile> {
    INSTANCE
        .get()
        .expect("ESBuild metafile instance not initialized")
        .clone()
}
