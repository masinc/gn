use clap::Parser;
use std::path::PathBuf;
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

    #[clap(value_name = "PATH|URL|STDIN", default_value = "-")]
    pub input: Input,
}
