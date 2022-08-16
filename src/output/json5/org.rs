use crate::output::{self, OutputWriter};

#[derive(Debug)]
pub struct Original;

impl OutputWriter<serde_json::Value> for Original {
    type Error = output::json::WriteError;
    type Context = output::json::Context;

    fn init_ctx(&self) -> Self::Context {
        output::json::Context::new_json5()
    }

    fn write_output(
        &mut self,
        writer: &mut dyn termcolor::WriteColor,
        value: &serde_json::Value,
        ctx: &mut Self::Context,
        config: &crate::output::Config,
    ) -> Result<(), Self::Error> {
        output::json::Original::new().write_output(writer, value, ctx, config)
    }
}
