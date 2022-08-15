use termcolor::WriteColor;

use crate::output::{
    json::{Context, WriteError},
    OutputWriter,
};

#[derive(Debug)]
pub struct JavaScript;

impl OutputWriter<serde_json::Value> for JavaScript {
    type Error = WriteError;
    type Context = Context;

    fn init_ctx(&self) -> Self::Context {
        todo!()
    }

    fn write_output(
        &mut self,
        writer: &mut dyn WriteColor,
        value: &serde_json::Value,
        ctx: &mut Self::Context,
        config: &crate::output::Config,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
