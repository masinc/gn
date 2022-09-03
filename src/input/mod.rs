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
    output::{Config, WriterKind},
};
use std::{error::Error as StdError, fmt::Display, io};

pub trait Input
where
    Self::DeserializeError: StdError + Sync + Send + 'static,
{
    type Value;
    type DeserializeError;

    fn id() -> &'static str;

    /// Returns a file extensions
    fn extensions() -> &'static [&'static str];

    /// deserialize string to Self::Value
    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError>;
}

pub trait WriteGron {
    type Error;
    fn write_gron(writer: &mut impl io::Write, s: &str, config: &Config)
        -> Result<(), Self::Error>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputType {
    Json,
    Json5,
    Toml,
    Yaml,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    fn write_gron(
        &self,
        writer: &mut impl io::Write,
        s: &str,
        config: &Config,
    ) -> anyhow::Result<()> {
        match self {
            InputType::Json => Json::write_gron(writer, s, config),
            InputType::Json5 => Json5::write_gron(writer, s, config),
            InputType::Toml => Toml::write_gron(writer, s, config),
            InputType::Yaml => Yaml::write_gron(writer, s, config),
        }
    }

    pub fn write(
        &self,
        writer: &mut impl io::Write,
        s: &str,
        kind: &WriterKind,
        config: &Config,
    ) -> anyhow::Result<()> {
        match kind {
            WriterKind::Gron => self.write_gron(writer, s, config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn test_ArgInputType_to_InputType() {
        use cli::ArgInputType;
        assert_eq!(
            InputType::try_from(ArgInputType::Auto),
            Err(InputTypeAutoError)
        );

        assert_eq!(InputType::try_from(ArgInputType::Json), Ok(InputType::Json));
        assert_eq!(
            InputType::try_from(ArgInputType::Json5),
            Ok(InputType::Json5)
        );
        assert_eq!(InputType::try_from(ArgInputType::Toml), Ok(InputType::Toml));
        assert_eq!(InputType::try_from(ArgInputType::Yaml), Ok(InputType::Yaml));
    }

    #[test]
    fn test_input_guess_by_extension() {
        assert_eq!(
            InputType::guess_by_extension("json"),
            Some(InputType::Json)
        );

        assert_eq!(
            InputType::guess_by_extension("json5"),
            Some(InputType::Json5)
        );

        assert_eq!(
            InputType::guess_by_extension("toml"),
            Some(InputType::Toml)
        );

        assert_eq!(
            InputType::guess_by_extension("yml"),
            Some(InputType::Yaml)
        );

        assert_eq!(
            InputType::guess_by_extension("yaml"),
            Some(InputType::Yaml)
        );

        assert_eq!(
            InputType::guess_by_extension("html"),
            None
        );

    }
}
