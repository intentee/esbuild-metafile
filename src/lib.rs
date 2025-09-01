pub mod asset;
mod filesystem;
pub mod filters;
pub mod http_preloader;
pub mod instance;
pub mod preloadable_asset;

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
pub use http_preloader::HttpPreloader;
use serde::Deserialize;

#[derive(Deserialize)]
struct EsbuildMetaFileLoader {
    outputs: HashMap<String, Output>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InputInOutput {}

#[derive(Clone, Debug, Deserialize)]
pub struct Output {
    imports: Vec<Import>,
    #[serde(rename = "cssBundle")]
    css_bundle: Option<String>,
    #[serde(rename = "entryPoint")]
    entry_point: Option<String>,
    #[serde(default)]
    inputs: HashMap<String, InputInOutput>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Import {
    path: String,
}

#[derive(Clone, Debug, Default)]
pub struct EsbuildMetaFile {
    input_to_outputs: Arc<HashMap<String, Vec<String>>>,
    output_to_preloads: Arc<HashMap<String, Vec<String>>>,
    static_paths: Arc<HashMap<String, Vec<String>>>,
}

impl EsbuildMetaFile {
    pub fn find_static_paths_for_input(&self, input_path: &str) -> Option<Vec<String>> {
        self.static_paths.get(input_path).cloned()
    }

    pub fn find_outputs_for_input(&self, input_path: &str) -> Option<Vec<String>> {
        self.input_to_outputs.get(input_path).cloned()
    }

    pub fn get_preloads(&self, output_path: &str) -> Vec<String> {
        self.output_to_preloads
            .get(output_path)
            .cloned()
            .unwrap_or_default()
    }
}

impl FromStr for EsbuildMetaFile {
    type Err = anyhow::Error;

    fn from_str(json: &str) -> Result<EsbuildMetaFile> {
        let metafile: EsbuildMetaFileLoader = serde_json::from_str(json)?;
        let mut input_to_outputs: HashMap<String, Vec<String>> = HashMap::new();
        let mut output_to_preloads: HashMap<String, Vec<String>> = HashMap::new();
        let mut static_paths: HashMap<String, Vec<String>> = HashMap::new();

        let mut remaining_outputs: HashSet<String> = metafile
            .outputs
            .keys()
            .filter(|path| !path.ends_with(".map"))
            .cloned()
            .collect();

        for (output_path, output) in &metafile.outputs {
            if let Some(entry_point) = &output.entry_point {
                remaining_outputs.remove(output_path);

                let outputs = input_to_outputs.entry(entry_point.clone()).or_default();
                let preloads = output_to_preloads.entry(output_path.clone()).or_default();

                outputs.push(output_path.clone());

                if let Some(css_bundle) = &output.css_bundle {
                    match metafile.outputs.get(css_bundle) {
                        Some(css_output) => {
                            remaining_outputs.remove(css_bundle);
                            outputs.push(css_bundle.clone());

                            for import in &css_output.imports {
                                remaining_outputs.remove(&import.path);
                                preloads.push(import.path.clone());
                            }
                        }
                        None => {
                            return Err(anyhow::anyhow!(
                                "CSS bundle '{}' not found in outputs",
                                css_bundle
                            ));
                        }
                    }
                }

                for import in &output.imports {
                    remaining_outputs.remove(&import.path);
                    preloads.push(import.path.clone());
                }
            } else {
                // Static files use the same extension as the input file, and no entry point
                for input_path in output.inputs.keys() {
                    static_paths
                        .entry(input_path.to_string())
                        .or_default()
                        .push(output_path.to_string());
                }
            }
        }

        if !remaining_outputs.is_empty() {
            return Err(anyhow::anyhow!(
                "Some outputs were not processed: {:?}",
                remaining_outputs
            ));
        }

        Ok(Self {
            input_to_outputs: Arc::new(input_to_outputs),
            output_to_preloads: Arc::new(output_to_preloads),
            static_paths: Arc::new(static_paths),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_metafile_fonts;
    use crate::test::get_metafile_svg;

    #[test]
    fn test_find_outputs_for_css_input() -> Result<()> {
        let metafile = get_metafile_fonts()?;
        let outputs = metafile
            .find_outputs_for_input("resources/css/page-common.css")
            .unwrap();

        assert_eq!(outputs.len(), 1);
        assert!(outputs.contains(&"static/page-common_DO3RNJ3I.css".to_string()));

        Ok(())
    }

    #[test]
    fn test_find_outputs_for_tsx_input() -> Result<()> {
        let metafile = get_metafile_fonts()?;
        let outputs = metafile
            .find_outputs_for_input("resources/ts/controller_foo.tsx")
            .unwrap();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains(&"static/controller_foo_CTJMZK66.js".to_string()));
        assert!(outputs.contains(&"static/controller_foo_CX2Z63ZH.css".to_string()));

        Ok(())
    }

    #[test]
    fn test_get_preloads_for_js() -> Result<()> {
        let metafile = get_metafile_fonts()?;
        let preloads = metafile.get_preloads("static/controller_foo_CTJMZK66.js");

        assert_eq!(preloads.len(), 5);
        assert!(preloads.contains(&"https://fonts/font1.woff2".to_string()));
        assert!(preloads.contains(&"https://fonts/font3.woff2".to_string()));
        assert!(preloads.contains(&"static/chunk-EMZKCXNJ.js".to_string()));
        assert!(preloads.contains(&"static/chunk-PI4ZFSEL.js".to_string()));
        assert!(preloads.contains(&"static/logo_XSTJPNLH.png".to_string()));

        Ok(())
    }

    #[test]
    fn test_get_preloads_for_css() -> Result<()> {
        let metafile = get_metafile_fonts()?;
        let preloads = metafile.get_preloads("static/page-common_DO3RNJ3I.css");

        assert_eq!(preloads.len(), 3);
        assert!(preloads.contains(&"https://fonts/font1.woff2".to_string()));
        assert!(preloads.contains(&"https://fonts/font2.woff2".to_string()));
        assert!(preloads.contains(&"static/test_6D5OPEBZ.svg".to_string()));

        Ok(())
    }

    #[test]
    fn test_get_file_path() -> Result<()> {
        let metafile = get_metafile_svg()?;
        let outputs = metafile
            .find_static_paths_for_input("resources/images/image.svg")
            .unwrap();

        assert_eq!(outputs.len(), 1);
        assert!(outputs.contains(&"dist/image_123.svg".to_string()));

        Ok(())
    }
}
