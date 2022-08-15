use crate::output::{
    yaml::{Context, WriteError},
    OutputWriter,
};

#[derive(Debug)]
pub struct JavaScript;

impl OutputWriter<::toml::Value> for JavaScript {
    type Error = WriteError;
    type Context = Context;

    fn init_ctx(&self) -> Self::Context {
        todo!()
    }

    fn write_output(
        &mut self,
        writer: &mut dyn termcolor::WriteColor,
        value: &::toml::Value,
        ctx: &mut Self::Context,
        config: &crate::output::Config,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
