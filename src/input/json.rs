use crate::{
    input::{Input, WriteGron},
    output,
};
use serde_json::{Error, Value};
use std::io;

#[derive(Debug)]
pub struct Json;

impl Input for Json {
    type Value = Value;
    type DeserializeError = Error;

    fn id() -> &'static str {
        "json"
    }

    fn extensions() -> &'static [&'static str] {
        &["json"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        serde_json::from_str(s)
    }
}

impl WriteGron for Json {
    type Error = anyhow::Error;

    fn write_gron(
        writer: &mut impl io::Write,
        s: &str,
        config: &output::Config,
    ) -> Result<(), Self::Error> {
        let v: serde_json::Value = Json::deserialize_str(s)?;

        serde_gron::to_writer_with(&v, writer, Self::id(), config.color.into())?;
        Ok(())
    }
}
