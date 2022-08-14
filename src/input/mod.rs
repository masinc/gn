pub mod json;
pub mod json5;
pub mod toml;
pub mod yaml;

pub use crate::input::{json::Json, json5::Json5, toml::Toml, yaml::Yaml};

use either::Either;

pub trait Input {
    type Context;
    type Value;
    type DeserializeError;
    type ParseError;

    fn id() -> &'static str;

    /// Returns a file extensions
    fn extensions() -> &'static [&'static str];

    /// initialize context
    fn init_ctx() -> Self::Context;

    /// deserialize string to Self::Value
    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError>;

    /// deserialized value to gron String
    fn parse<'a: 'c, 'b, 'c>(
        ctx: &'a mut Self::Context,
        v: &'b Self::Value,
    ) -> Result<&'c str, Self::ParseError>;

    /// String to gron String
    fn to_gron(
        s: impl AsRef<str>,
    ) -> Result<String, Either<Self::DeserializeError, Self::ParseError>> {
        let v = Self::deserialize_str(s.as_ref());

        let v = match v {
            Ok(v) => v,
            Err(e) => return Err(Either::Left(e)),
        };

        let mut ctx = Self::init_ctx();

        let r = match Self::parse(&mut ctx, &v) {
            Ok(r) => r,
            Err(e) => return Err(Either::Right(e)),
        };
        Ok(r.to_string())
    }
}

pub enum InputType {
    Json,
    Json5,
    Toml,
    Yaml,
}

// #[derive(thiserror::Error, Debug)]
// pub enum ParseError {
//     #[error(transparent)]
//     JsonDeserialize(::serde_json::Error),
//     #[error(transparent)]
//     JsonParse(crate::input::json::ParseError),
//     #[error(transparent)]
//     Json5Deserialize(::json5::Error),
//     #[error(transparent)]
//     Json5Parse(crate::input::json::ParseError),
//     #[error(transparent)]
//     TomlDeserialize(::toml::de::Error),
//     #[error(transparent)]
//     TomlParseError(crate::input::toml::ParseError),
//     #[error(transparent)]
//     YamlDeserialize(::serde_yaml::Error),
//     #[error(transparent)]
//     YamlParseError(crate::input::yaml::ParseError),
// }

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

    pub fn to_gron(&self, s: &str) -> anyhow::Result<String> {
        match self {
            InputType::Json => Ok(Json::to_gron(s)?),
            InputType::Json5 => Ok(Json5::to_gron(s)?),
            InputType::Toml => Ok(Toml::to_gron(s)?),
            InputType::Yaml => Ok(Yaml::to_gron(s)?),
        }
    }

    pub fn guess_and_to_gron(s: &str) -> Option<String> {
        // guess json
        if let Ok(s) = InputType::Json.to_gron(s) {
            return Some(s);
        }

        // guess json5
        if let Ok(s) = InputType::Json5.to_gron(s) {
            return Some(s);
        }

        // guess toml
        if let Ok(s) = InputType::Toml.to_gron(s) {
            return Some(s);
        }

        // guess yaml
        if let Ok(s) = InputType::Yaml.to_gron(s) {
            return Some(s);
        }

        None
    }
}
