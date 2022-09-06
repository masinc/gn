use crate::cli;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn test_ArgColor_to_Color() {
        use cli::ArgColor;

        assert_eq!(Color::from(ArgColor::Always), Color::Colored);
        assert_eq!(Color::from(ArgColor::Never), Color::Regular);
        // Depends on the situation.
        // assert_eq!(Color::from(ArgColor::Auto), Color::Colored);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_Color_to_FormatType() {
        use serde_gron::FormatType;

        assert_eq!(FormatType::Color, FormatType::from(Color::Colored));
        assert_eq!(FormatType::Regular, FormatType::from(Color::Regular))
    }
}
