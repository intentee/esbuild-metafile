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
                "<script async src=\"/{}\" type=\"module\"></script>",
                renders_path.render_path(path)
            ),
            Asset::Stylesheet(path) => format!(
                "<link rel=\"stylesheet\" href=\"/{}\">",
                renders_path.render_path(path)
            ),
            Asset::Unknown(_) => String::new(),
        }
    }
}
