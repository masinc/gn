pub mod json;
pub mod json5;
pub mod toml;
pub mod yaml;

use std::io;
use termcolor::{ColorSpec, WriteColor};

#[derive(Debug)]
pub struct Config {}

pub trait OutputWriter<Value> {
    type Error;
    type Context;

    /// initialize context
    fn init_ctx(&self) -> Self::Context;

    fn write_output(
        &mut self,
        writer: &mut dyn WriteColor,
        value: &Value,
        ctx: &mut Self::Context,
        config: &Config,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum WriterKind {
    Original,
    JavaScript,
}

impl Default for WriterKind {
    fn default() -> Self {
        WriterKind::Original
    }
}

pub trait WriteColorExt: WriteColor {
    fn write_color(&mut self, color: &ColorSpec, s: &str) -> io::Result<()> {
        self.set_color(color)?;
        write!(self, "{s}")?;
        self.reset()
    }

    fn writeln_color(&mut self, color: &ColorSpec, s: &str) -> io::Result<()> {
        self.write_color(color, s)?;
        writeln!(self)
    }
}

impl WriteColorExt for &mut dyn WriteColor {}
