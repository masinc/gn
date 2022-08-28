use crate::cli;
use std::io;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Regular,
    Colored,
}

impl From<cli::ArgColor> for Color {
    fn from(c: cli::ArgColor) -> Self {
        match c {
            cli::ArgColor::Auto => {
                if atty::is(atty::Stream::Stdout) {
                    Color::Colored
                } else {
                    Color::Regular
                }
            }
            cli::ArgColor::Always => Color::Colored,
            cli::ArgColor::Never => Color::Regular,
        }
    }
}

impl From<Color> for serde_gron::FormatType {
    fn from(val: Color) -> Self {
        use serde_gron::FormatType;
        match val {
            Color::Regular => FormatType::Regular,
            Color::Colored => FormatType::Color,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub color: Color,
}

pub trait OutputWriter<Value> {
    type Error;
    type Context;

    /// initialize context
    fn init_ctx(&self) -> Self::Context;

    fn write_output(
        &mut self,
        writer: &mut impl io::Write,
        value: &Value,
        ctx: &mut Self::Context,
        config: &Config,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum WriterKind {
    Gron,
}

impl Default for WriterKind {
    fn default() -> Self {
        WriterKind::Gron
    }
}
