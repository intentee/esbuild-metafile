use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::filesystem::get_file_extension;

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

    fn prefix_path(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("/{path}")
        }
    }
}

impl Display for PreloadableAsset {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PreloadableAsset::Fetch(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"{}\" as=\"fetch\">",
                self.prefix_path(path),
            ),
            PreloadableAsset::Font(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"{}\" as=\"font\" crossorigin>",
                self.prefix_path(path),
            ),
            PreloadableAsset::Image(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"{}\" as=\"image\">",
                self.prefix_path(path),
            ),
            PreloadableAsset::Module(path) => writeln!(
                formatter,
                "<link rel=\"modulepreload\" href=\"{}\">",
                self.prefix_path(path),
            ),
            PreloadableAsset::Stylesheet(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"{}\" as=\"style\">",
                self.prefix_path(path),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_local_font_formatting() -> Result<()> {
        let font = PreloadableAsset::Font("fonts/Roboto.woff2".to_string());
        let expected =
            "<link rel=\"preload\" href=\"/fonts/Roboto.woff2\" as=\"font\" crossorigin>\n";

        assert_eq!(format!("{font}"), expected);

        Ok(())
    }

    #[test]
    fn test_url_font_formatting() -> Result<()> {
        let font =
            PreloadableAsset::Font("https://fonts.somewhere.com/fonts/Roboto.woff2".to_string());
        let expected = "<link rel=\"preload\" href=\"https://fonts.somewhere.com/fonts/Roboto.woff2\" as=\"font\" crossorigin>\n";

        assert_eq!(format!("{font}"), expected);

        Ok(())
    }
}
