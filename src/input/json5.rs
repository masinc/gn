use crate::{
    input::Input,
    output::{self, WriterKind},
};
use serde_json::Value;
use std::io;

#[derive(Debug)]
pub struct Json5;

impl Input for Json5 {
    type Value = Value;
    type DeserializeError = json5::Error;
    type ParseError = output::json::WriteError;
    type Context = output::json::Context;
    type WriteError = io::Error;

    fn id() -> &'static str {
        "json5"
    }

    fn extensions() -> &'static [&'static str] {
        &["json5"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        json5::from_str(s)
    }

    fn output_writer(
        kind: &WriterKind,
    ) -> Box<
        dyn crate::output::OutputWriter<
            Self::Value,
            Error = Self::WriteError,
            Context = Self::Context,
        >,
    > {
        match kind {
            WriterKind::Original => Box::new(output::json5::Original),
            WriterKind::JavaScript => todo!(),
        }
    }
}
