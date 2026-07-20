use crate::output_properties::OutputProperties;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputLookup {
    Found(OutputProperties),
    NotFound,
}
