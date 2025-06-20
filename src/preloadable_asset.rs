use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::filesystem::get_file_extension;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
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
}

impl Display for PreloadableAsset {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PreloadableAsset::Fetch(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"/{path}\" as=\"fetch\">"
            ),
            PreloadableAsset::Font(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"/{path}\" as=\"font\" crossorigin>"
            ),
            PreloadableAsset::Image(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"/{path}\" as=\"image\">"
            ),
            PreloadableAsset::Module(path) => {
                writeln!(formatter, "<link rel=\"modulepreload\" href=\"/{path}\">")
            }
            PreloadableAsset::Stylesheet(path) => writeln!(
                formatter,
                "<link rel=\"preload\" href=\"/{path}\" as=\"style\">"
            ),
        }
    }
}
