use crate::{
    input::{Input, WriteGron},
    output::{self},
};
use serde_json::Value;
use std::io;

#[derive(Debug)]
pub struct Json5;

impl Input for Json5 {
    type Value = Value;
    type DeserializeError = json5::Error;

    fn id() -> &'static str {
        "json5"
    }

    fn extensions() -> &'static [&'static str] {
        &["json5"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        json5::from_str(s)
    }
}

impl WriteGron for Json5 {
    type Error = anyhow::Error;

    fn write_gron(
        writer: &mut impl io::Write,
        s: &str,
        config: &output::Config,
    ) -> Result<(), Self::Error> {
        let v: serde_json::Value = Json5::deserialize_str(s)?;
        serde_gron::to_writer_with(&v, writer, Self::id(), config.color.into())?;
        Ok(())
    }
}
