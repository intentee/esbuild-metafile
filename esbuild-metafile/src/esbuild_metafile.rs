use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::error::Error;
use crate::import::Import;
use crate::output::Output;
use crate::raw_esbuild_metafile::RawEsbuildMetafile;

fn register_preloads_for_output<'preloads>(
    metafile: &'preloads RawEsbuildMetafile,
    outputs: &'preloads mut Vec<String>,
    preloads: &'preloads mut Vec<String>,
    remaining_outputs: &'preloads mut HashSet<String>,
    output_path: &'preloads str,
) {
    if let Some(output) = metafile.outputs.get(output_path) {
        remaining_outputs.remove(output_path);

        let output_path_str = output_path.to_string();

        if !outputs.contains(&output_path_str) {
            outputs.push(output_path_str);

            register_preloads_from_imports(
                metafile,
                outputs,
                preloads,
                remaining_outputs,
                &output.imports,
            );
        }
    }
}

fn register_preloads_from_imports<'preloads>(
    metafile: &'preloads RawEsbuildMetafile,
    outputs: &'preloads mut Vec<String>,
    preloads: &'preloads mut Vec<String>,
    remaining_outputs: &'preloads mut HashSet<String>,
    imports: &'preloads [Import],
) {
    for Import {
        path,
    } in imports
    {
        if !preloads.contains(path) {
            remaining_outputs.remove(path);
            preloads.push(path.clone());

            register_preloads_for_output(metafile, outputs, preloads, remaining_outputs, path);
        }
    }
}

#[derive(Debug, Default)]
pub struct EsbuildMetafile {
    input_to_outputs: HashMap<String, Vec<String>>,
    output_paths: HashSet<String>,
    output_to_preloads: HashMap<String, Vec<String>>,
    static_paths: HashMap<String, Vec<String>>,
}

impl EsbuildMetafile {
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
}

impl From<RawEsbuildMetafile> for EsbuildMetafile {
    fn from(metafile: RawEsbuildMetafile) -> EsbuildMetafile {
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
            if let Some(entry_point) = &entry_point {
                remaining_outputs.remove(output_path);

                let outputs = input_to_outputs.entry(entry_point.clone()).or_default();
                let preloads = output_to_preloads.entry(output_path.clone()).or_default();

                outputs.push(output_path.clone());

                if let Some(css_bundle) = css_bundle {
                    register_preloads_for_output(
                        &metafile,
                        outputs,
                        preloads,
                        &mut remaining_outputs,
                        css_bundle,
                    );
                }

                register_preloads_from_imports(
                    &metafile,
                    outputs,
                    preloads,
                    &mut remaining_outputs,
                    imports,
                );
            } else {
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
            log::warn!("Some outputs were not processed: {remaining_outputs:?}");
        }

        Self {
            input_to_outputs,
            output_paths: metafile
                .outputs
                .keys()
                .map(|key| key.to_string())
                .collect::<HashSet<String>>(),
            output_to_preloads,
            static_paths,
        }
    }
}

impl FromStr for EsbuildMetafile {
    type Err = Error;

    fn from_str(json: &str) -> Result<EsbuildMetafile, Error> {
        Ok(serde_json::from_str::<RawEsbuildMetafile>(json)?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::get_metafile_basic;
    use crate::test::get_metafile_dedup;
    use crate::test::get_metafile_fonts;
    use crate::test::get_metafile_glb;
    use crate::test::get_metafile_orphan;
    use crate::test::get_metafile_svg;

    #[test]
    fn test_get_output_paths() {
        let metafile = get_metafile_basic();
        let outputs = metafile.get_output_paths();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains("dist/main.css"));
        assert!(outputs.contains("dist/main.js"));
    }

    #[test]
    fn test_find_outputs_for_css_input() {
        let metafile = get_metafile_fonts();
        let outputs = metafile
            .find_outputs_for_input("resources/css/page-common.css")
            .unwrap();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains(&"static/page-common_DO3RNJ3I.css".to_string()));
        assert!(outputs.contains(&"static/test_6D5OPEBZ.svg".to_string()));
    }

    #[test]
    fn test_find_outputs_for_tsx_input() {
        let metafile = get_metafile_fonts();
        let outputs = metafile
            .find_outputs_for_input("resources/ts/controller_foo.tsx")
            .unwrap();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains(&"static/controller_foo_CTJMZK66.js".to_string()));
        assert!(outputs.contains(&"static/controller_foo_CX2Z63ZH.css".to_string()));
    }

    #[test]
    fn test_get_preloads_for_js() {
        let metafile = get_metafile_fonts();
        let preloads = metafile.get_preloads("static/controller_foo_CTJMZK66.js");

        assert_eq!(preloads.len(), 5);
        assert!(preloads.contains(&"https://fonts/font1.woff2".to_string()));
        assert!(preloads.contains(&"https://fonts/font3.woff2".to_string()));
        assert!(preloads.contains(&"static/chunk-EMZKCXNJ.js".to_string()));
        assert!(preloads.contains(&"static/chunk-PI4ZFSEL.js".to_string()));
        assert!(preloads.contains(&"static/logo_XSTJPNLH.png".to_string()));
    }

    #[test]
    fn test_get_preloads_for_css() {
        let metafile = get_metafile_fonts();
        let preloads = metafile.get_preloads("static/page-common_DO3RNJ3I.css");

        assert_eq!(preloads.len(), 3);
        assert!(preloads.contains(&"https://fonts/font1.woff2".to_string()));
        assert!(preloads.contains(&"https://fonts/font2.woff2".to_string()));
        assert!(preloads.contains(&"static/test_6D5OPEBZ.svg".to_string()));
    }

    #[test]
    fn test_get_preloads_for_unknown_output_is_empty() {
        let metafile = get_metafile_basic();

        assert!(metafile.get_preloads("dist/does-not-exist.js").is_empty());
    }

    #[test]
    fn test_get_file_path_for_glb() {
        let metafile = get_metafile_glb();
        let outputs = metafile
            .find_static_paths_for_input("resources/media/models/model.glb")
            .unwrap();

        assert_eq!(outputs.len(), 1);
        assert!(outputs.contains(&"dist/model_123.glb".to_string()));

        let preloads = metafile.get_preloads("dist/main.js");

        assert_eq!(preloads.len(), 3);
        assert!(preloads.contains(&"dist/chunk-ABC.js".to_string()));
        assert!(preloads.contains(&"dist/chunk-DEF.js".to_string()));
        assert!(preloads.contains(&"dist/model_123.glb".to_string()));
    }

    #[test]
    fn test_get_file_path_for_svg() {
        let metafile = get_metafile_svg();
        let outputs = metafile
            .find_static_paths_for_input("resources/images/image.svg")
            .unwrap();

        assert_eq!(outputs.len(), 1);
        assert!(outputs.contains(&"dist/image_123.svg".to_string()));
    }

    #[test]
    fn test_orphan_output_is_listed_but_unmapped() {
        let metafile = get_metafile_orphan();

        assert!(metafile.get_output_paths().contains("dist/orphan.js"));
        assert!(metafile.find_outputs_for_input("dist/orphan.js").is_none());
        assert!(
            metafile
                .find_static_paths_for_input("dist/orphan.js")
                .is_none()
        );
    }

    #[test]
    fn test_from_str_rejects_invalid_json() {
        let error = EsbuildMetafile::from_str("not valid json").unwrap_err();

        assert!(matches!(error, Error::Deserialize(_)));
        assert_eq!(error.to_string(), "failed to deserialize esbuild metafile");
        assert!(std::error::Error::source(&error).is_some());
    }

    #[test]
    fn test_shared_output_referenced_twice_is_registered_once() {
        let metafile = get_metafile_dedup();
        let outputs = metafile.find_outputs_for_input("src/entry.ts").unwrap();

        assert_eq!(outputs.len(), 2);
        assert_eq!(
            outputs
                .iter()
                .filter(|path| *path == "dist/shared.js")
                .count(),
            1
        );
        assert!(outputs.contains(&"dist/entry.js".to_string()));
    }
}
