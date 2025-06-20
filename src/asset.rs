use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::filesystem::get_file_extension;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
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
}

impl Display for Asset {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Asset::Script(path) => writeln!(
                formatter,
                "<script async src=\"/{path}\" type=\"module\"></script>"
            ),
            Asset::Stylesheet(path) => {
                writeln!(formatter, "<link rel=\"stylesheet\" href=\"/{path}\">")
            }
            Asset::Unknown(_) => write!(formatter, ""),
        }
    }
}
