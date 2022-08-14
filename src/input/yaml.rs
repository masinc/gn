use crate::input::Input;
use serde_yaml::{Error, Value};
use std::{convert::Infallible, fmt::Write};

pub trait ToYamlKey {
    fn to_yaml_key(&self) -> String;
}

impl ToYamlKey for serde_yaml::Sequence {
    fn to_yaml_key(&self) -> String {
        let mut s = String::new();
        s.push('[');
        for (i, v) in self.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&v.to_yaml_key());
        }
        s.push(']');
        s
    }
}

impl ToYamlKey for serde_yaml::Mapping {
    fn to_yaml_key(&self) -> String {
        let mut s = String::new();
        s.push('{');
        for (i, (k, v)) in self.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            write!(&mut s, "{}: {}", k.to_yaml_key(), v.to_yaml_key()).unwrap();
        }
        s.push('}');

        s
    }
}

impl ToYamlKey for String {
    fn to_yaml_key(&self) -> String {
        match self.as_str() {
            "null" | "on" | "off" => {
                format!("\"{}\"", self)
            }
            s if s.find(|c| !char::is_ascii_alphanumeric(&c)).is_some() => {
                format!("\"{}\"", self)
            }

            s if s.parse::<isize>().is_ok() | s.parse::<bool>().is_ok() => {
                format!("\"{}\"", self)
            }

            _ => self.clone(),
        }
    }
}

impl ToYamlKey for serde_yaml::Value {
    fn to_yaml_key(&self) -> String {
        match self {
            Value::Null => "null".into(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_yaml_key(),
            Value::Sequence(a) => a.to_yaml_key(),
            Value::Mapping(m) => m.to_yaml_key(),
            Value::Tagged(t) => {
                let tag = t.tag.to_string();
                let k = t.value.to_yaml_key();

                format!("{tag} {k}")
            }
        }
    }
}

#[derive(Debug)]
pub enum YamlPath {
    /// array index
    Array(usize),
    /// map key
    Map(String),
}

#[derive(Debug, Default)]
pub struct YamlContext {
    path: Vec<YamlPath>,
    result: String,
}

impl YamlContext {
    fn path_string(&self) -> String {
        let mut s = String::new();
        s.push_str("yaml");

        for v in &self.path {
            match v {
                YamlPath::Array(n) => write!(&mut s, "[{n}]"),
                YamlPath::Map(k) => {
                    write!(&mut s, ".{k}")
                }
            }
            .unwrap();
        }

        s
    }
}

pub type ParseError = Infallible;

#[derive(Debug)]
pub struct Yaml;

impl Input for Yaml {
    type Context = YamlContext;
    type Value = Value;
    type DeserializeError = Error;
    type ParseError = ParseError;

    fn id() -> &'static str {
        "yaml"
    }

    fn extensions() -> &'static [&'static str] {
        &["yaml", "yml"]
    }

    fn init_ctx() -> Self::Context {
        Default::default()
    }

    fn deserialize_str(s: &str) -> Result<Self::Value, Self::DeserializeError> {
        serde_yaml::from_str(s)
    }

    fn parse<'a: 'c, 'b, 'c>(
        ctx: &'a mut Self::Context,
        v: &'b Self::Value,
    ) -> Result<&'c str, Self::ParseError> {
        let path = ctx.path_string();
        match v {
            serde_yaml::Value::Null => writeln!(ctx.result, "{path} = null",).unwrap(),
            serde_yaml::Value::Bool(b) => writeln!(ctx.result, "{path} = {b}").unwrap(),
            serde_yaml::Value::Number(n) => writeln!(ctx.result, "{path} = {n}").unwrap(),
            serde_yaml::Value::String(s) => writeln!(ctx.result, "{path} = \"{s}\"").unwrap(),
            serde_yaml::Value::Sequence(a) => {
                writeln!(ctx.result, "{path} = []").unwrap();
                for (i, x) in a.iter().enumerate() {
                    ctx.path.push(YamlPath::Array(i));
                    Self::parse(ctx, x)?;
                    ctx.path.pop();
                }
            }
            serde_yaml::Value::Mapping(m) => {
                writeln!(ctx.result, "{path} = {{}}").unwrap();
                for (k, v) in m {
                    ctx.path.push(YamlPath::Map(k.to_yaml_key()));
                    Self::parse(ctx, v)?;
                    ctx.path.pop();
                }
            }
            serde_yaml::Value::Tagged(t) => {
                writeln!(ctx.result, "{path} = {}", t.tag).unwrap();
                Self::parse(ctx, &t.value)?;
            }
        };

        Ok(&ctx.result)
    }
}
