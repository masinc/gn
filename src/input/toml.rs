use crate::input::{Input, WriteGron};
use crate::output;
use std::io;
use toml::{de::Error, Value};

#[derive(Debug)]
pub struct Toml;

impl Input for Toml {
    type Value = Value;
    type DeserializeError = Error;

    fn id() -> &'static str {
        "toml"
    }

    fn extensions() -> &'static [&'static str] {
        &["toml"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        toml::from_str(s)
    }
}

impl WriteGron for Toml {
    type Error = anyhow::Error;

    fn write_gron(
        writer: &mut impl io::Write,
        s: &str,
        config: &output::Config,
    ) -> Result<(), Self::Error> {
        let v: ::toml::Value = Toml::deserialize_str(s)?;
        serde_gron::to_writer_with(&v, writer, Self::id(), serde_gron::FormatType::Color)?;
        Ok(())
    }
}
