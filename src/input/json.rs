use crate::{
    input::Input,
    output::{self, OutputWriter, WriterKind},
};
use serde_json::{Error, Value};
use std::convert::Infallible;

#[derive(Debug)]
pub struct Json;

impl Input for Json {
    type Value = Value;
    type DeserializeError = Error;
    type ParseError = Infallible;
    type Context = output::json::Context;
    type WriteError = output::json::WriteError;

    fn id() -> &'static str {
        "json"
    }

    fn extensions() -> &'static [&'static str] {
        &["json"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        serde_json::from_str(s)
    }

    fn output_writer(
        kind: &WriterKind,
    ) -> Box<dyn OutputWriter<Self::Value, Error = Self::WriteError, Context = Self::Context>> {
        match kind {
            WriterKind::Original => Box::new(output::json::Original),
            WriterKind::JavaScript => Box::new(output::json::JavaScript),
        }
    }
}
