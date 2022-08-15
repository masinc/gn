use crate::input::Input;
use crate::output;
use serde_yaml::{Error, Value};
use std::convert::Infallible;

#[derive(Debug)]
pub struct Yaml;

impl Input for Yaml {
    type Value = Value;
    type DeserializeError = Error;
    type ParseError = Infallible;
    type Context = output::yaml::Context;
    type WriteError = output::yaml::WriteError;

    fn id() -> &'static str {
        "yaml"
    }

    fn extensions() -> &'static [&'static str] {
        &["yaml", "yml"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        serde_yaml::from_str(s)
    }

    fn output_writer(
        kind: &output::WriterKind,
    ) -> Box<dyn output::OutputWriter<Self::Value, Error = Self::WriteError, Context = Self::Context>>
    {
        match kind {
            output::WriterKind::Original => Box::new(output::yaml::Original),
            output::WriterKind::JavaScript => todo!(),
        }
    }
}
