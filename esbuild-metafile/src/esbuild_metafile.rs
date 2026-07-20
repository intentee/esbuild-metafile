use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::error::Error;
use crate::import::Import;
use crate::input_lookup::InputLookup;
use crate::input_properties::InputProperties;
use crate::output::Output;
use crate::output_lookup::OutputLookup;
use crate::output_properties::OutputProperties;
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
    pub fn input(&self, input_path: &str) -> InputLookup {
        let outputs = self.input_to_outputs.get(input_path).cloned();
        let static_paths = self.static_paths.get(input_path).cloned();

        match (outputs, static_paths) {
            (None, None) => InputLookup::NotFound,
            (outputs, static_paths) => InputLookup::Found(InputProperties {
                outputs: outputs.unwrap_or_default(),
                static_paths: static_paths.unwrap_or_default(),
            }),
        }
    }

    pub fn output(&self, output_path: &str) -> OutputLookup {
        if self.output_paths.contains(output_path) {
            OutputLookup::Found(OutputProperties {
                preloads: self
                    .output_to_preloads
                    .get(output_path)
                    .cloned()
                    .unwrap_or_default(),
            })
        } else {
            OutputLookup::NotFound
        }
    }

    pub fn get_output_paths(&self) -> HashSet<String> {
        self.output_paths.clone()
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

    fn found_input(lookup: InputLookup) -> Option<InputProperties> {
        match lookup {
            InputLookup::Found(input) => Some(input),
            InputLookup::NotFound => None,
        }
    }

    fn found_output(lookup: OutputLookup) -> Option<OutputProperties> {
        match lookup {
            OutputLookup::Found(output) => Some(output),
            OutputLookup::NotFound => None,
        }
    }

    #[test]
    fn test_get_output_paths() {
        let metafile = get_metafile_basic();
        let outputs = metafile.get_output_paths();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains("dist/main.css"));
        assert!(outputs.contains("dist/main.js"));
    }

    #[test]
    fn test_input_returns_outputs_for_css_input() {
        let metafile = get_metafile_fonts();
        let input = found_input(metafile.input("resources/css/page-common.css"))
            .expect("expected input to be found");

        assert_eq!(input.outputs.len(), 2);
        assert!(
            input
                .outputs
                .contains(&"static/page-common_DO3RNJ3I.css".to_string())
        );
        assert!(
            input
                .outputs
                .contains(&"static/test_6D5OPEBZ.svg".to_string())
        );
    }

    #[test]
    fn test_input_returns_outputs_for_tsx_input() {
        let metafile = get_metafile_fonts();
        let input = found_input(metafile.input("resources/ts/controller_foo.tsx"))
            .expect("expected input to be found");

        assert_eq!(input.outputs.len(), 2);
        assert!(
            input
                .outputs
                .contains(&"static/controller_foo_CTJMZK66.js".to_string())
        );
        assert!(
            input
                .outputs
                .contains(&"static/controller_foo_CX2Z63ZH.css".to_string())
        );
    }

    #[test]
    fn test_output_returns_preloads_for_js() {
        let metafile = get_metafile_fonts();
        let output = found_output(metafile.output("static/controller_foo_CTJMZK66.js"))
            .expect("expected output to be found");

        assert_eq!(output.preloads.len(), 5);
        assert!(
            output
                .preloads
                .contains(&"https://fonts/font1.woff2".to_string())
        );
        assert!(
            output
                .preloads
                .contains(&"https://fonts/font3.woff2".to_string())
        );
        assert!(
            output
                .preloads
                .contains(&"static/chunk-EMZKCXNJ.js".to_string())
        );
        assert!(
            output
                .preloads
                .contains(&"static/chunk-PI4ZFSEL.js".to_string())
        );
        assert!(
            output
                .preloads
                .contains(&"static/logo_XSTJPNLH.png".to_string())
        );
    }

    #[test]
    fn test_output_returns_preloads_for_css() {
        let metafile = get_metafile_fonts();
        let output = found_output(metafile.output("static/page-common_DO3RNJ3I.css"))
            .expect("expected output to be found");

        assert_eq!(output.preloads.len(), 3);
        assert!(
            output
                .preloads
                .contains(&"https://fonts/font1.woff2".to_string())
        );
        assert!(
            output
                .preloads
                .contains(&"https://fonts/font2.woff2".to_string())
        );
        assert!(
            output
                .preloads
                .contains(&"static/test_6D5OPEBZ.svg".to_string())
        );
    }

    #[test]
    fn test_output_is_not_found_for_unknown_output() {
        let metafile = get_metafile_basic();

        assert!(found_output(metafile.output("dist/does-not-exist.js")).is_none());
    }

    #[test]
    fn test_input_returns_static_paths_for_glb() {
        let metafile = get_metafile_glb();
        let input = found_input(metafile.input("resources/media/models/model.glb"))
            .expect("expected input to be found");

        assert_eq!(input.static_paths.len(), 1);
        assert!(
            input
                .static_paths
                .contains(&"dist/model_123.glb".to_string())
        );

        let output =
            found_output(metafile.output("dist/main.js")).expect("expected output to be found");

        assert_eq!(output.preloads.len(), 3);
        assert!(output.preloads.contains(&"dist/chunk-ABC.js".to_string()));
        assert!(output.preloads.contains(&"dist/chunk-DEF.js".to_string()));
        assert!(output.preloads.contains(&"dist/model_123.glb".to_string()));
    }

    #[test]
    fn test_input_returns_static_paths_for_svg() {
        let metafile = get_metafile_svg();
        let input = found_input(metafile.input("resources/images/image.svg"))
            .expect("expected input to be found");

        assert_eq!(input.static_paths.len(), 1);
        assert!(
            input
                .static_paths
                .contains(&"dist/image_123.svg".to_string())
        );
    }

    #[test]
    fn test_orphan_output_is_listed_but_unmapped() {
        let metafile = get_metafile_orphan();

        assert!(metafile.get_output_paths().contains("dist/orphan.js"));
        assert!(found_input(metafile.input("dist/orphan.js")).is_none());
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
        let input =
            found_input(metafile.input("src/entry.ts")).expect("expected input to be found");

        assert_eq!(input.outputs.len(), 2);
        assert_eq!(
            input
                .outputs
                .iter()
                .filter(|path| *path == "dist/shared.js")
                .count(),
            1
        );
        assert!(input.outputs.contains(&"dist/entry.js".to_string()));
    }
}
