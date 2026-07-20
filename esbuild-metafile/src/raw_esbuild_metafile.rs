use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::output::Output;

/// The raw esbuild `--metafile` JSON. Build the query-optimized
/// [`EsbuildMetafile`](crate::esbuild_metafile::EsbuildMetafile) from it with [`From`]/[`Into`].
#[derive(Debug, Deserialize, Serialize)]
pub struct RawEsbuildMetafile {
    pub outputs: HashMap<String, Output>,
}
