use crate::filesystem::get_file_extension;
use crate::renders_path::RendersPath;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PreloadableAsset {
    Fetch(String),
    Font(String),
    Image(String),
    Stylesheet(String),
    Module(String),
}

impl PreloadableAsset {
    pub fn from_path(path: String) -> Self {
        match get_file_extension(&path) {
            Some("js") => PreloadableAsset::Module(path),
            Some("css") => PreloadableAsset::Stylesheet(path),
            Some("woff") | Some("woff2") | Some("ttf") | Some("otf") => {
                PreloadableAsset::Font(path)
            }
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("webp")
            | Some("avif") | Some("svg") => PreloadableAsset::Image(path),
            _ => PreloadableAsset::Fetch(path),
        }
    }

    pub fn render<TRendersPath: RendersPath>(&self, renders_path: &TRendersPath) -> String {
        match self {
            PreloadableAsset::Fetch(path) => format!(
                "<link rel=\"preload\" href=\"{}\" as=\"fetch\" crossorigin>",
                renders_path.render_path(path),
            ),
            PreloadableAsset::Font(path) => format!(
                "<link rel=\"preload\" href=\"{}\" as=\"font\" crossorigin>",
                renders_path.render_path(path),
            ),
            PreloadableAsset::Image(path) => format!(
                "<link rel=\"preload\" href=\"{}\" as=\"image\">",
                renders_path.render_path(path),
            ),
            PreloadableAsset::Module(path) => format!(
                "<link rel=\"modulepreload\" href=\"{}\">",
                renders_path.render_path(path),
            ),
            PreloadableAsset::Stylesheet(path) => format!(
                "<link rel=\"preload\" href=\"{}\" as=\"style\">",
                renders_path.render_path(path),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path_renderer::PathRenderer;

    #[test]
    fn test_local_font_formatting() {
        let font =
            PreloadableAsset::Font("fonts/Roboto.woff2".to_string()).render(&PathRenderer {});

        assert_eq!(
            font,
            "<link rel=\"preload\" href=\"/fonts/Roboto.woff2\" as=\"font\" crossorigin>"
        );
    }

    #[test]
    fn test_url_font_formatting() {
        let font =
            PreloadableAsset::Font("https://fonts.somewhere.com/fonts/Roboto.woff2".to_string())
                .render(&PathRenderer {});

        assert_eq!(
            font,
            "<link rel=\"preload\" href=\"https://fonts.somewhere.com/fonts/Roboto.woff2\" as=\"font\" crossorigin>"
        );
    }

    #[test]
    fn test_module_from_path_renders_modulepreload() {
        let module =
            PreloadableAsset::from_path("dist/app.js".to_string()).render(&PathRenderer {});

        assert_eq!(module, "<link rel=\"modulepreload\" href=\"/dist/app.js\">");
    }

    #[test]
    fn test_stylesheet_from_path_renders_style_preload() {
        let stylesheet =
            PreloadableAsset::from_path("dist/app.css".to_string()).render(&PathRenderer {});

        assert_eq!(
            stylesheet,
            "<link rel=\"preload\" href=\"/dist/app.css\" as=\"style\">"
        );
    }

    #[test]
    fn test_all_image_extensions_render_image_preload() {
        for extension in ["png", "jpg", "jpeg", "gif", "webp", "avif", "svg"] {
            let path = format!("dist/logo.{extension}");
            let image = PreloadableAsset::from_path(path.clone()).render(&PathRenderer {});

            assert_eq!(
                image,
                format!("<link rel=\"preload\" href=\"/{path}\" as=\"image\">")
            );
        }
    }

    #[test]
    fn test_extensionless_path_renders_fetch_preload() {
        let fetch = PreloadableAsset::from_path("dist/data".to_string()).render(&PathRenderer {});

        assert_eq!(
            fetch,
            "<link rel=\"preload\" href=\"/dist/data\" as=\"fetch\" crossorigin>"
        );
    }
}
