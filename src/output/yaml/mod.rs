mod js;
mod org;

pub use js::JavaScript;
pub use org::Original;
use std::{fmt::Write, io};

pub type WriteError = io::Error;

pub trait ToYamlKey {
    fn to_yaml_key(&self) -> String;
}

#[derive(Debug)]
pub enum YamlPath {
    /// array index
    Array(usize),
    /// map key
    Map(String),
}

#[derive(Debug, Default)]
pub struct Context {
    path: Vec<YamlPath>,
}

impl Context {
    fn path_string(&self) -> String {
        let mut s = String::new();
        s.push_str("yaml");

        for v in &self.path {
            match v {
                YamlPath::Array(n) => write!(&mut s, "[{n}]"),
                YamlPath::Map(k) => {
                    write!(&mut s, ".{k}")
                }
            }
            .unwrap();
        }

        s
    }
}
