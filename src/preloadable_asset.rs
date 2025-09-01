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
                "<link rel=\"preload\" href=\"{}\" as=\"fetch\">",
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
    use anyhow::Result;

    use super::*;
    use crate::path_renderer::PathRenderer;

    #[test]
    fn test_local_font_formatting() -> Result<()> {
        let path_renderer = PathRenderer {};
        let font = PreloadableAsset::Font("fonts/Roboto.woff2".to_string()).render(&path_renderer);
        let expected =
            "<link rel=\"preload\" href=\"/fonts/Roboto.woff2\" as=\"font\" crossorigin>";

        assert_eq!(format!("{font}"), expected);

        Ok(())
    }

    #[test]
    fn test_url_font_formatting() -> Result<()> {
        let path_renderer = PathRenderer {};
        let font =
            PreloadableAsset::Font("https://fonts.somewhere.com/fonts/Roboto.woff2".to_string())
                .render(&path_renderer);
        let expected = "<link rel=\"preload\" href=\"https://fonts.somewhere.com/fonts/Roboto.woff2\" as=\"font\" crossorigin>";

        assert_eq!(format!("{font}"), expected);

        Ok(())
    }
}
