use crate::input::Input;
use std::convert::Infallible;
use std::fmt::Write;
use toml::{de::Error, Value};

#[derive(Debug)]
pub enum TomlPath {
    /// array index
    Array(usize),
    /// map key
    Object(String),
}

#[derive(Debug, Default)]
pub struct YamlContext {
    path: Vec<TomlPath>,
    result: String,
}

impl YamlContext {
    fn path_string(&self) -> String {
        let mut s = String::new();
        s.push_str("toml");

        for v in &self.path {
            match v {
                TomlPath::Array(n) => write!(&mut s, "[{n}]"),
                TomlPath::Object(k) => write!(&mut s, ".{k}"),
            }
            .unwrap()
        }

        s
    }
}

pub type ParseError = Infallible;

#[derive(Debug)]
pub struct Toml;

impl Input for Toml {
    type Context = YamlContext;
    type Value = Value;
    type DeserializeError = Error;
    type ParseError = ParseError;

    fn id() -> &'static str {
        "toml"
    }

    fn extensions() -> &'static [&'static str] {
        &["toml"]
    }

    fn init_ctx() -> Self::Context {
        Default::default()
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        toml::from_str(s)
    }

    fn parse<'a: 'c, 'b, 'c>(
        ctx: &'a mut Self::Context,
        v: &'b Self::Value,
    ) -> Result<&'c str, Self::ParseError> {
        let path = ctx.path_string();

        match v {
            Value::String(s) => writeln!(ctx.result, "{path} = \"{s}\"").unwrap(),
            Value::Integer(n) => writeln!(ctx.result, "{path} = {n}").unwrap(),
            Value::Float(n) => writeln!(ctx.result, "{path} = {n}").unwrap(),
            Value::Boolean(b) => writeln!(ctx.result, "{path} = {b}").unwrap(),
            Value::Datetime(d) => writeln!(ctx.result, "{path} = {d}").unwrap(),
            Value::Array(a) => {
                writeln!(ctx.result, "{path} = []").unwrap();
                for (i, x) in a.iter().enumerate() {
                    ctx.path.push(TomlPath::Array(i));
                    Self::parse(ctx, x)?;
                    ctx.path.pop();
                }
            }
            Value::Table(t) => {
                writeln!(ctx.result, "{path} = {{}}").unwrap();
                for (k, v) in t.iter() {
                    ctx.path.push(TomlPath::Object(k.clone()));
                    Self::parse(ctx, v)?;
                    ctx.path.pop();
                }
            }
        }

        Ok(&ctx.result)
    }
}
