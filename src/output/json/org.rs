use crate::output::{
    json::{Context, JsonPath},
    OutputWriter,
};
use std::io;
use termcolor::WriteColor;

#[derive(Debug)]
pub struct Original;

impl OutputWriter<serde_json::Value> for Original {
    type Error = io::Error;
    type Context = Context;

    fn init_ctx(&self) -> Self::Context {
        Default::default()
    }

    fn write_output(
        &mut self,
        writer: &mut dyn WriteColor,
        value: &serde_json::Value,
        ctx: &mut Self::Context,
        config: &crate::output::Config,
    ) -> Result<(), Self::Error> {
        let path = ctx.path_string();
        match value {
            serde_json::Value::Null => writeln!(writer, "{path} = null")?,
            serde_json::Value::Bool(b) => writeln!(writer, "{path} = {}", b)?,
            serde_json::Value::Number(n) => writeln!(writer, "{path} = {}", n)?,
            serde_json::Value::String(s) => writeln!(writer, "{path} = \"{}\"", s)?,
            serde_json::Value::Array(a) => {
                writeln!(writer, "{path} = []").unwrap();
                for (i, x) in a.iter().enumerate() {
                    ctx.path.push(JsonPath::Array(i));
                    self.write_output(writer, x, ctx, config)?;
                    ctx.path.pop();
                }
            }
            serde_json::Value::Object(o) => {
                writeln!(writer, "{path} = {{}}")?;

                for (k, v) in o.iter() {
                    ctx.path.push(JsonPath::Object(k.to_string()));
                    self.write_output(writer, v, ctx, config)?;
                    ctx.path.pop();
                }
            }
        };

        Ok(())
    }
}
