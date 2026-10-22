use std::collections::BTreeMap;
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

fn push_unique(paths: &mut Vec<String>, path: &str) -> bool {
    if paths.iter().any(|existing_path| existing_path == path) {
        return false;
    }

    paths.push(path.to_string());

    true
}

fn register_preloads_for_output<'preloads>(
    metafile: &'preloads RawEsbuildMetafile,
    reached_outputs: &'preloads mut Vec<String>,
    preloads: &'preloads mut Vec<String>,
    remaining_outputs: &'preloads mut HashSet<String>,
    output_path: &'preloads str,
) {
    if let Some(output) = metafile.outputs.get(output_path) {
        remaining_outputs.remove(output_path);

        if push_unique(reached_outputs, output_path) {
            register_preloads_from_imports(
                metafile,
                reached_outputs,
                preloads,
                remaining_outputs,
                &output.imports,
            );
        }
    }
}

fn register_preloads_from_imports<'preloads>(
    metafile: &'preloads RawEsbuildMetafile,
    reached_outputs: &'preloads mut Vec<String>,
    preloads: &'preloads mut Vec<String>,
    remaining_outputs: &'preloads mut HashSet<String>,
    imports: &'preloads [Import],
) {
    for Import {
        path,
    } in imports
    {
        if push_unique(preloads, path) {
            remaining_outputs.remove(path);

            register_preloads_for_output(
                metafile,
                reached_outputs,
                preloads,
                remaining_outputs,
                path,
            );
        }
    }
}

#[derive(Debug, Default)]
pub struct EsbuildMetafile {
    inputs: HashMap<String, InputProperties>,
    output_paths: HashSet<String>,
    output_to_preloads: HashMap<String, Vec<String>>,
}

impl EsbuildMetafile {
    pub fn input(&self, input_path: &str) -> InputLookup {
        match self.inputs.get(input_path) {
            Some(input) => InputLookup::Found(input.clone()),
            None => InputLookup::NotFound,
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
        let mut inputs: HashMap<String, InputProperties> = HashMap::new();
        let mut output_to_preloads: HashMap<String, Vec<String>> = HashMap::new();

        let mut remaining_outputs: HashSet<String> = metafile
            .outputs
            .keys()
            .filter(|path| !path.ends_with(".map"))
            .cloned()
            .collect();

        let outputs_in_path_order: BTreeMap<&String, &Output> = metafile.outputs.iter().collect();

        for (
            output_path,
            Output {
                imports,
                css_bundle,
                entry_point,
                inputs: output_inputs,
            },
        ) in outputs_in_path_order
        {
            if let Some(entry_point) = &entry_point {
                remaining_outputs.remove(output_path);

                let mut reached_outputs: Vec<String> = vec![output_path.clone()];
                let mut preloads: Vec<String> = Vec::new();

                if let Some(css_bundle) = css_bundle {
                    register_preloads_for_output(
                        &metafile,
                        &mut reached_outputs,
                        &mut preloads,
                        &mut remaining_outputs,
                        css_bundle,
                    );
                }

                register_preloads_from_imports(
                    &metafile,
                    &mut reached_outputs,
                    &mut preloads,
                    &mut remaining_outputs,
                    imports,
                );

                let input = inputs.entry(entry_point.clone()).or_default();

                for reached_output in &reached_outputs {
                    push_unique(&mut input.outputs, reached_output);
                }

                for preload in &preloads {
                    push_unique(&mut input.preloads, preload);
                }

                output_to_preloads.insert(output_path.clone(), preloads);
            } else {
                for input_path in output_inputs.keys() {
                    remaining_outputs.remove(output_path);
                    inputs
                        .entry(input_path.to_string())
                        .or_default()
                        .static_paths
                        .push(output_path.to_string());
                }
            }
        }

        if !remaining_outputs.is_empty() {
            log::warn!("Some outputs were not processed: {remaining_outputs:?}");
        }

        Self {
            inputs,
            output_paths: metafile
                .outputs
                .keys()
                .map(|key| key.to_string())
                .collect::<HashSet<String>>(),
            output_to_preloads,
        }
    }
}

impl FromStr for EsbuildMetafile {
    type Err = Error;

    fn from_str(json: &str) -> Result<EsbuildMetafile, Error> {
        Ok(serde_json::from_str::<RawEsbuildMetafile>(json)?.into())
    }
}
