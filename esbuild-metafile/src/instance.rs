use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::OnceCell;

use super::EsbuildMetaFile;

pub static INSTANCE: OnceCell<Arc<EsbuildMetaFile>> = OnceCell::new();

pub fn create_from_contents(esbuild_meta_contents: &str) -> Result<EsbuildMetaFile> {
    EsbuildMetaFile::from_str(esbuild_meta_contents)
}

pub fn initialize_instance(esbuild_meta_contents: &str) {
    let esbuild_metafile = Arc::new(
        create_from_contents(esbuild_meta_contents)
            .expect("Unable to create ESBuild metafile instance"),
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
