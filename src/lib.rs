pub mod asset;
mod filesystem;
pub mod filters;
pub mod http_preloader;
pub mod instance;
pub mod path_renderer;
pub mod preloadable_asset;
pub mod renders_path;

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;
use anyhow::anyhow;
pub use http_preloader::HttpPreloader;
use serde::Deserialize;

#[derive(Deserialize)]
struct EsbuildMetaFileLoader {
    outputs: HashMap<String, Output>,
}

#[derive(Debug, Deserialize)]
pub struct InputInOutput {}

#[derive(Debug, Deserialize)]
pub struct Output {
    imports: Vec<Import>,
    #[serde(rename = "cssBundle")]
    css_bundle: Option<String>,
    #[serde(rename = "entryPoint")]
    entry_point: Option<String>,
    #[serde(default)]
    inputs: HashMap<String, InputInOutput>,
}

#[derive(Debug, Deserialize)]
pub struct Import {
    path: String,
}

#[derive(Debug, Default)]
pub struct EsbuildMetaFile {
    input_to_outputs: HashMap<String, Vec<String>>,
    output_paths: HashSet<String>,
    output_to_preloads: HashMap<String, Vec<String>>,
    static_paths: HashMap<String, Vec<String>>,
}

impl EsbuildMetaFile {
    pub fn find_static_paths_for_input(&self, input_path: &str) -> Option<Vec<String>> {
        self.static_paths.get(input_path).cloned()
    }

    pub fn find_outputs_for_input(&self, input_path: &str) -> Option<Vec<String>> {
        self.input_to_outputs.get(input_path).cloned()
    }

    pub fn get_output_paths(&self) -> HashSet<String> {
        self.output_paths.clone()
    }

    pub fn get_preloads(&self, output_path: &str) -> Vec<String> {
        self.output_to_preloads
            .get(output_path)
            .cloned()
            .unwrap_or_default()
    }

    fn register_preloads_for_output<'preloads>(
        metafile: &'preloads EsbuildMetaFileLoader,
        outputs: &'preloads mut Vec<String>,
        preloads: &'preloads mut Vec<String>,
        remaining_outputs: &'preloads mut HashSet<String>,
        output_path: &'preloads str,
    ) -> Result<()> {
        if let Some(output) = metafile.outputs.get(output_path) {
            remaining_outputs.remove(output_path);
            outputs.push(output_path.to_string());

            Self::register_preloads_from_imports(
                metafile,
                outputs,
                preloads,
                remaining_outputs,
                &output.imports,
            )?;
        }

        Ok(())
    }

    fn register_preloads_from_imports<'preloads>(
        metafile: &'preloads EsbuildMetaFileLoader,
        outputs: &'preloads mut Vec<String>,
        preloads: &'preloads mut Vec<String>,
        remaining_outputs: &'preloads mut HashSet<String>,
        imports: &'preloads [Import],
    ) -> Result<()> {
        for Import {
            path,
        } in imports
        {
            if !preloads.contains(path) {
                remaining_outputs.remove(path);
                println!("preload: {}", path);
                preloads.push(path.clone());

                Self::register_preloads_for_output(
                    metafile,
                    outputs,
                    preloads,
                    remaining_outputs,
                    path,
                )?;
            }
        }

        Ok(())
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

        for (
            output_path,
            Output {
                imports,
                css_bundle,
                entry_point,
                inputs,
            },
        ) in &metafile.outputs
        {
            println!("output path: {output_path}");
            if let Some(entry_point) = &entry_point {
                remaining_outputs.remove(output_path);

                let outputs = input_to_outputs.entry(entry_point.clone()).or_default();
                let preloads = output_to_preloads.entry(output_path.clone()).or_default();

                outputs.push(output_path.clone());

                if let Some(css_bundle) = css_bundle {
                    Self::register_preloads_for_output(
                        &metafile,
                        outputs,
                        preloads,
                        &mut remaining_outputs,
                        css_bundle,
                    )?;
                }

                Self::register_preloads_from_imports(
                    &metafile,
                    outputs,
                    preloads,
                    &mut remaining_outputs,
                    imports,
                )?;
            } else {
                // Static files use the same extension as the input file, and no entry point
                for input_path in inputs.keys() {
                    remaining_outputs.remove(output_path);
                    static_paths
                        .entry(input_path.to_string())
                        .or_default()
                        .push(output_path.to_string());
                }
            }
        }

        if !remaining_outputs.is_empty() {
            return Err(anyhow!(
                "Some outputs were not processed: {remaining_outputs:?}"
            ));
        }

        Ok(Self {
            input_to_outputs,
            output_paths: metafile
                .outputs
                .keys()
                .map(|key| key.to_string())
                .collect::<HashSet<String>>(),
            output_to_preloads,
            static_paths,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_metafile_basic;
    use crate::test::get_metafile_fonts;
    use crate::test::get_metafile_glb;
    use crate::test::get_metafile_svg;

    #[test]
    fn test_get_output_paths() -> Result<()> {
        let metafile = get_metafile_basic()?;
        let outputs = metafile.get_output_paths();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains("dist/main.css"));
        assert!(outputs.contains("dist/main.js"));

        Ok(())
    }

    #[test]
    fn test_find_outputs_for_css_input() -> Result<()> {
        let metafile = get_metafile_fonts()?;
        let outputs = metafile
            .find_outputs_for_input("resources/css/page-common.css")
            .unwrap();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains(&"static/page-common_DO3RNJ3I.css".to_string()));
        assert!(outputs.contains(&"static/test_6D5OPEBZ.svg".to_string()));

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
    fn test_get_file_path_for_glb() -> Result<()> {
        let metafile = get_metafile_glb()?;
        let outputs = metafile
            .find_static_paths_for_input("resources/media/models/model.glb")
            .unwrap();

        assert_eq!(outputs.len(), 1);
        assert!(outputs.contains(&"dist/model_123.glb".to_string()));

        let preloads = metafile.get_preloads("dist/main.js");

        println!("preloads: {preloads:?}");

        assert_eq!(preloads.len(), 3);
        assert!(preloads.contains(&"dist/chunk-ABC.js".to_string()));
        assert!(preloads.contains(&"dist/chunk-DEF.js".to_string()));
        assert!(preloads.contains(&"dist/model_123.glb".to_string()));

        Ok(())
    }

    #[test]
    fn test_get_file_path_for_svg() -> Result<()> {
        let metafile = get_metafile_svg()?;
        let outputs = metafile
            .find_static_paths_for_input("resources/images/image.svg")
            .unwrap();

        assert_eq!(outputs.len(), 1);
        assert!(outputs.contains(&"dist/image_123.svg".to_string()));

        Ok(())
    }
}
