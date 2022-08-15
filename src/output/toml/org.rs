use toml::Value;

use crate::output::{
    toml::{Context, TomlPath, WriteError},
    OutputWriter,
};

#[derive(Debug)]
pub struct Original;

impl OutputWriter<toml::Value> for Original {
    type Error = WriteError;
    type Context = Context;

    fn init_ctx(&self) -> Self::Context {
        Default::default()
    }

    fn write_output(
        &mut self,
        writer: &mut dyn termcolor::WriteColor,
        value: &toml::Value,
        ctx: &mut Self::Context,
        config: &crate::output::Config,
    ) -> Result<(), Self::Error> {
        let path = ctx.path_string();

        match value {
            Value::String(s) => writeln!(writer, "{path} = \"{s}\"")?,
            Value::Integer(n) => writeln!(writer, "{path} = {n}")?,
            Value::Float(n) => writeln!(writer, "{path} = {n}")?,
            Value::Boolean(b) => writeln!(writer, "{path} = {b}")?,
            Value::Datetime(d) => writeln!(writer, "{path} = {d}")?,
            Value::Array(a) => {
                writeln!(writer, "{path} = []")?;
                for (i, x) in a.iter().enumerate() {
                    ctx.path.push(TomlPath::Array(i));
                    self.write_output(writer, x, ctx, config)?;
                    ctx.path.pop();
                }
            }
            Value::Table(t) => {
                writeln!(writer, "{path} = {{}}")?;
                for (k, v) in t.iter() {
                    ctx.path.push(TomlPath::Object(k.clone()));
                    self.write_output(writer, v, ctx, config)?;
                    ctx.path.pop();
                }
            }
        };
        Ok(())
    }
}
