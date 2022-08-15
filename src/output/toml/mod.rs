mod js;
mod org;

pub use js::JavaScript;
pub use org::Original;

use std::{fmt::Write, io};

pub type WriteError = io::Error;

#[derive(Debug)]
pub enum TomlPath {
    /// array index
    Array(usize),
    /// map key
    Object(String),
}

#[derive(Debug, Default)]
pub struct Context {
    path: Vec<TomlPath>,
}

impl Context {
    fn path_string(&self) -> String {
        let mut s = String::new();
        s.push_str("toml");

        for v in &self.path {
            match v {
                TomlPath::Array(n) => write!(&mut s, "[{n}]"),
                TomlPath::Object(k) => write!(&mut s, ".{k}"),
            }
            .unwrap()
        }

        s
    }
}
