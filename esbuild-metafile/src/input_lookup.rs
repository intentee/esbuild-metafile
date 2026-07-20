use crate::input_properties::InputProperties;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InputLookup {
    Found(InputProperties),
    NotFound,
}
