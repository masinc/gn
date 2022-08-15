pub mod json;
pub mod json5;
pub mod toml;
pub mod yaml;

pub use crate::input::json::Json;
pub use crate::input::json5::Json5;
pub use crate::input::toml::Toml;
pub use crate::input::yaml::Yaml;
use crate::{
    cli,
    output::{Config, OutputWriter, WriterKind},
};
use std::{error::Error as StdError, fmt::Display};
use termcolor::WriteColor;

pub trait Input
where
    Self::DeserializeError: StdError + Sync + Send + 'static,
    Self::ParseError: StdError + Sync + Send + 'static,
    Self::WriteError: StdError + Sync + Send + 'static,
{
    type Value;
    type DeserializeError;
    type ParseError;
    type Context;
    type WriteError;

    fn id() -> &'static str;

    /// Returns a file extensions
    fn extensions() -> &'static [&'static str];

    /// deserialize string to Self::Value
    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError>;

    fn write(
        writer: &mut dyn WriteColor,
        s: &str,
        kind: &WriterKind,
        config: &Config,
    ) -> anyhow::Result<()> {
        let value: Self::Value = Self::deserialize_str(s)?;
        let mut output_writer = Self::output_writer(kind);

        let mut ctx = output_writer.init_ctx();

        output_writer.write_output(writer, &value, &mut ctx, config)?;

        Ok(())
    }

    fn output_writer(
        kind: &WriterKind,
    ) -> Box<dyn OutputWriter<Self::Value, Error = Self::WriteError, Context = Self::Context>>;
}

#[derive(Debug, Copy, Clone)]
pub enum InputType {
    Json,
    Json5,
    Toml,
    Yaml,
}

#[derive(Debug)]
pub struct InputTypeAutoError;

impl Display for InputTypeAutoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", stringify!(InputTypeAutoError))
    }
}

impl StdError for InputTypeAutoError {}

impl TryFrom<cli::ArgInputType> for InputType {
    type Error = InputTypeAutoError;

    fn try_from(value: cli::ArgInputType) -> Result<Self, Self::Error> {
        match value {
            cli::ArgInputType::Auto => Err(InputTypeAutoError),
            cli::ArgInputType::Json => Ok(InputType::Json),
            cli::ArgInputType::Json5 => Ok(InputType::Json5),
            cli::ArgInputType::Yaml => Ok(InputType::Yaml),
            cli::ArgInputType::Toml => Ok(InputType::Toml),
        }
    }
}

impl InputType {
    pub fn guess_by_extension(ext: impl AsRef<str>) -> Option<Self> {
        match ext.as_ref() {
            x if Json::extensions().contains(&x) => Some(Self::Json),
            x if Json5::extensions().contains(&x) => Some(Self::Json5),
            x if Toml::extensions().contains(&x) => Some(Self::Toml),
            x if Yaml::extensions().contains(&x) => Some(Self::Yaml),
            _ => None,
        }
    }

    pub fn write(
        &self,
        writer: &mut dyn WriteColor,
        s: &str,
        kind: &WriterKind,
        config: &Config,
    ) -> anyhow::Result<()> {
        match self {
            InputType::Json => Json::write(writer, s, kind, config),
            InputType::Json5 => Json5::write(writer, s, kind, config),
            InputType::Toml => Toml::write(writer, s, kind, config),
            InputType::Yaml => Yaml::write(writer, s, kind, config),
        }
    }
}
