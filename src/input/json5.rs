use serde_json::Value;

use crate::input::{json, Input};

#[derive(Debug)]
pub struct Json5;

impl Input for Json5 {
    type Context = json::JsonContext;
    type Value = Value;
    type DeserializeError = json5::Error;
    type ParseError = json::ParseError;

    fn id() -> &'static str {
        "json5"
    }

    fn extensions() -> &'static [&'static str] {
        &["json5"]
    }

    fn init_ctx() -> Self::Context {
        json::JsonContext::new_json5()
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        json5::from_str(s)
    }

    fn parse<'a: 'c, 'b, 'c>(
        ctx: &'a mut Self::Context,
        v: &'b Self::Value,
    ) -> Result<&'c str, Self::ParseError> {
        json::Json::parse(ctx, v)
    }
}
