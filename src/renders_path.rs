pub trait RendersPath {
    fn render_path(&self, path: &str) -> String;
}
