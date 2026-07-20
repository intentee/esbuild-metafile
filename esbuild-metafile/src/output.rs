use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::import::Import;
use crate::input_in_output::InputInOutput;

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    pub imports: Vec<Import>,
    #[serde(rename = "cssBundle")]
    pub css_bundle: Option<String>,
    #[serde(rename = "entryPoint")]
    pub entry_point: Option<String>,
    #[serde(default)]
    pub inputs: HashMap<String, InputInOutput>,
}
