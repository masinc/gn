use clap::Parser;
use std::{
    fs,
    io::{self, prelude::*},
    path::{Path, PathBuf},
};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum ArgInputType {
    Auto,
    Json,
    Json5,
    Toml,
    Yaml,
}

impl Default for ArgInputType {
    fn default() -> Self {
        Self::Auto
    }
}

impl From<crate::input::InputType> for ArgInputType {
    fn from(input_type: crate::input::InputType) -> Self {
        match input_type {
            crate::input::InputType::Json => ArgInputType::Json,
            crate::input::InputType::Json5 => ArgInputType::Json5,
            crate::input::InputType::Toml => ArgInputType::Toml,
            crate::input::InputType::Yaml => ArgInputType::Yaml,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Stdin,
    Url(Url),
    Path(PathBuf),
}

impl Input {
    pub fn extension(&self) -> Option<String> {
        match self {
            Input::Stdin => None,
            Input::Url(url) => Path::new(url.path())
                .extension()
                .and_then(|s| s.to_str())
                .map(String::from),
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_input_extension() -> anyhow::Result<()> {
        assert_eq!(None, Input::Stdin.extension());
        assert_eq!(
            Some("html".to_string()),
            Input::Url(Url::parse("https://example.com/index.html")?).extension()
        );
        assert_eq!(
            Some("json".to_string()),
            Input::Path(PathBuf::from_str("/data/package.json")?).extension()
        );
        Ok(())
    }
}
