use crate::input::{Input, WriteGron};
use crate::output;
use serde_yaml::{Error, Value};

#[derive(Debug)]
pub struct Yaml;

impl Input for Yaml {
    type Value = Value;
    type DeserializeError = Error;

    fn id() -> &'static str {
        "yaml"
    }

    fn extensions() -> &'static [&'static str] {
        &["yaml", "yml"]
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        serde_yaml::from_str(s)
    }
}

impl WriteGron for Yaml {
    type Error = anyhow::Error;

    fn write_gron(
        writer: &mut impl std::io::Write,
        s: &str,
        config: &output::Config,
    ) -> Result<(), Self::Error> {
        let v: serde_yaml::Value = Yaml::deserialize_str(s)?;
        serde_gron::to_writer_with(&v, writer, Self::id(), serde_gron::FormatType::Color)?;
        Ok(())
    }
}
