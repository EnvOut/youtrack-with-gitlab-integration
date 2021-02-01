use std::fmt;

#[derive(Debug)]
pub enum DefinitionError {
    CanIdentifyType,
    UnsupportedType(String),
}