#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to deserialize esbuild metafile")]
    Deserialize(#[from] serde_json::Error),
}
