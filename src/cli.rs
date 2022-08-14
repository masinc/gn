use clap::Parser;
use std::{
    fs,
    io::{self, prelude::*},
    path::PathBuf,
};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ArgInputType {
    Auto,
    Json,
    Json5,
    Yaml,
    Toml,
}

impl Default for ArgInputType {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ArgColor {
    Auto,
    Always,
    Never,
}

impl Default for ArgColor {
    fn default() -> Self {
        ArgColor::Auto
    }
}

impl ArgColor {
    pub fn color_choice(&self) -> termcolor::ColorChoice {
        use atty::Stream;
        use termcolor::ColorChoice;
        match self {
            ArgColor::Auto => {
                if atty::is(Stream::Stdout) {
                    ColorChoice::Always
                } else {
                    ColorChoice::Never
                }
            }
            ArgColor::Always => ColorChoice::Always,
            ArgColor::Never => ColorChoice::Never,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Stdin,
    Url(Url),
    Path(PathBuf),
}

impl Input {
    pub fn extension(&self) -> Option<String> {
        match self {
            Input::Stdin => None,
            Input::Url(u) => unimplemented!(),
            Input::Path(p) => p.extension().and_then(|s| s.to_str().map(String::from)),
        }
    }

    pub fn read(&self) -> io::Result<Box<dyn Read>> {
        match self {
            Input::Stdin => Ok(Box::new(io::stdin().lock())),
            Input::Path(s) => Ok(Box::new(fs::File::open(s)?)),
            Input::Url(_) => todo!(),
        }
    }
}

impl std::str::FromStr for Input {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "-" => Ok(Input::Stdin),
            s => {
                if let Ok(url) = Url::parse(s) {
                    Ok(Input::Url(url))
                } else {
                    Ok(Input::Path(PathBuf::from(s)))
                }
            }
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(value_enum, short, long, default_value_t)]
    pub input_type: ArgInputType,

    #[clap(value_enum, short, long, default_value_t)]
    pub color: ArgColor,

    #[clap(value_name = "PATH|URL|STDIN", default_value = "-")]
    pub input: Input,
}
