use crate::filesystem::get_file_extension;
use crate::renders_path::RendersPath;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Asset {
    Stylesheet(String),
    Script(String),
    Unknown(String),
}

impl Asset {
    pub fn from_path(path: String) -> Self {
        match get_file_extension(&path) {
            Some("js") => Asset::Script(path),
            Some("css") => Asset::Stylesheet(path),
            _ => Asset::Unknown(path),
        }
    }

    pub fn render<TRendersPath: RendersPath>(&self, renders_path: &TRendersPath) -> String {
        match self {
            Asset::Script(path) => format!(
                "<script async src=\"{}\" type=\"module\"></script>",
                renders_path.render_path(path)
            ),
            Asset::Stylesheet(path) => format!(
                "<link rel=\"stylesheet\" href=\"{}\">",
                renders_path.render_path(path)
            ),
            Asset::Unknown(_) => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path_renderer::PathRenderer;

    #[test]
    fn test_script_renders_module_script_tag() {
        let rendered = Asset::from_path("dist/app.js".to_string()).render(&PathRenderer {});

        assert_eq!(
            rendered,
            "<script async src=\"/dist/app.js\" type=\"module\"></script>"
        );
    }

    #[test]
    fn test_stylesheet_renders_stylesheet_link() {
        let rendered = Asset::from_path("dist/app.css".to_string()).render(&PathRenderer {});

        assert_eq!(rendered, "<link rel=\"stylesheet\" href=\"/dist/app.css\">");
    }

    #[test]
    fn test_unknown_renders_empty_string() {
        let rendered = Asset::from_path("dist/model.bin".to_string()).render(&PathRenderer {});

        assert_eq!(rendered, "");
    }
}
