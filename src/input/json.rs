use crate::input::Input;
use serde_json::{Error, Value};
use std::{convert::Infallible, fmt::Write};

#[derive(Debug, Clone)]
pub enum JsonPath {
    /// array index
    Array(usize),
    /// object key
    Object(String),
}

#[derive(Debug)]
pub struct JsonContext {
    root: String,
    path: Vec<JsonPath>,
    result: String,
}

impl JsonContext {
    pub fn new() -> Self {
        Self {
            root: "json".into(),
            path: Default::default(),
            result: Default::default(),
        }
    }

    pub fn new_json5() -> Self {
        Self {
            root: "json5".into(),
            ..Default::default()
        }
    }
}

impl Default for JsonContext {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonContext {
    pub fn path_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.root);

        for v in &self.path {
            match v {
                JsonPath::Array(n) => write!(&mut s, "[{n}]").unwrap(),
                JsonPath::Object(k) => write!(&mut s, ".{k}").unwrap(),
            };
        }

        s
    }
}

pub type ParseError = Infallible;

#[derive(Debug)]
pub struct Json;

impl Input for Json {
    type Context = JsonContext;
    type Value = Value;
    type DeserializeError = Error;
    type ParseError = ParseError;

    fn id() -> &'static str {
        "json"
    }

    fn extensions() -> &'static [&'static str] {
        &["json"]
    }

    fn init_ctx() -> Self::Context {
        Default::default()
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        serde_json::from_str(s)
    }

    fn parse<'a: 'c, 'b, 'c>(
        ctx: &'a mut Self::Context,
        v: &'b Self::Value,
    ) -> Result<&'c str, Self::ParseError> {
        let path = ctx.path_string();
        match v {
            serde_json::Value::Null => writeln!(ctx.result, "{path} = null").unwrap(),
            serde_json::Value::Bool(b) => writeln!(ctx.result, "{path} = {}", b).unwrap(),
            serde_json::Value::Number(n) => writeln!(ctx.result, "{path} = {}", n).unwrap(),
            serde_json::Value::String(s) => writeln!(ctx.result, "{path} = \"{}\"", s).unwrap(),
            serde_json::Value::Array(a) => {
                writeln!(ctx.result, "{path} = []").unwrap();
                for (i, x) in a.iter().enumerate() {
                    ctx.path.push(JsonPath::Array(i));
                    let _ = Self::parse(ctx, x);
                    ctx.path.pop();
                }
            }
            serde_json::Value::Object(o) => {
                writeln!(ctx.result, "{path} = {{}}").unwrap();

                for (k, v) in o.iter() {
                    ctx.path.push(JsonPath::Object(k.to_string()));
                    let _ = Self::parse(ctx, v);
                    ctx.path.pop();
                }
            }
        };

        Ok(&ctx.result)
    }
}
