use crate::input::Input;
use crate::output::{self, WriterKind};
use std::{convert::Infallible, io};
use toml::{de::Error, Value};

#[derive(Debug)]
pub struct Toml;

impl Input for Toml {
    type Value = Value;
    type DeserializeError = Error;
    type ParseError = Infallible;
    type Context = output::toml::Context;
    type WriteError = io::Error;

    fn id() -> &'static str {
        "toml"
    }

    fn extensions() -> &'static [&'static str] {
        &["toml"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        toml::from_str(s)
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
            WriterKind::Original => Box::new(output::toml::Original),
            WriterKind::JavaScript => todo!(),
        }
    }
}
