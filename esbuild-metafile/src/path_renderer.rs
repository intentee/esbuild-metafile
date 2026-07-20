use crate::renders_path::RendersPath;

pub struct PathRenderer {}

impl RendersPath for PathRenderer {
    fn render_path(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("/{path}")
        }
    }
}
