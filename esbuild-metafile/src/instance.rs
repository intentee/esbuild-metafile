use std::str::FromStr;
use std::sync::Arc;

use once_cell::sync::OnceCell;

use crate::error::Error;
use crate::esbuild_metafile::EsbuildMetafile;

static INSTANCE: OnceCell<Arc<EsbuildMetafile>> = OnceCell::new();

pub fn create_from_contents(esbuild_meta_contents: &str) -> Result<EsbuildMetafile, Error> {
    EsbuildMetafile::from_str(esbuild_meta_contents)
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

pub fn get_esbuild_metafile() -> Arc<EsbuildMetafile> {
    INSTANCE
        .get()
        .expect("ESBuild metafile instance not initialized")
        .clone()
}
