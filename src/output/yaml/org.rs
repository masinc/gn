use crate::output::yaml::{Context, ToYamlKey, WriteError, YamlPath};
use crate::output::OutputWriter;
use serde_yaml::Value;
use std::fmt::Write;

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
pub struct Original;

impl OutputWriter<serde_yaml::Value> for Original {
    type Error = WriteError;
    type Context = Context;

    fn init_ctx(&self) -> Self::Context {
        Default::default()
    }

    fn write_output(
        &mut self,
        writer: &mut dyn termcolor::WriteColor,
        value: &serde_yaml::Value,
        ctx: &mut Self::Context,
        config: &crate::output::Config,
    ) -> Result<(), Self::Error> {
        let path = ctx.path_string();
        match value {
            Value::Null => writeln!(writer, "{path} = null",)?,
            Value::Bool(b) => writeln!(writer, "{path} = {b}")?,
            Value::Number(n) => writeln!(writer, "{path} = {n}")?,
            Value::String(s) => writeln!(writer, "{path} = \"{s}\"")?,
            Value::Sequence(a) => {
                writeln!(writer, "{path} = []")?;
                for (i, x) in a.iter().enumerate() {
                    ctx.path.push(YamlPath::Array(i));
                    self.write_output(writer, x, ctx, config)?;
                    ctx.path.pop();
                }
            }
            Value::Mapping(m) => {
                writeln!(writer, "{path} = {{}}")?;
                for (k, v) in m {
                    ctx.path.push(YamlPath::Map(k.to_yaml_key()));
                    self.write_output(writer, v, ctx, config)?;
                    ctx.path.pop();
                }
            }
            serde_yaml::Value::Tagged(t) => {
                writeln!(writer, "{path} = {}", t.tag).unwrap();
                self.write_output(writer, &t.value, ctx, config)?;
            }
        };

        Ok(())
    }
}
