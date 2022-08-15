mod js;
mod org;

pub use js::JavaScript;
pub use org::Original;

use std::{fmt::Write, io};

pub type WriteError = io::Error;

#[derive(Debug)]
pub struct Context {
    root: String,
    path: Vec<JsonPath>,
}

#[derive(Debug, Clone)]
pub enum JsonPath {
    /// array index
    Array(usize),
    /// object key
    Object(String),
}

impl Context {
    pub fn new() -> Self {
        Self {
            root: "json".into(),
            path: Default::default(),
        }
    }

    pub fn new_json5() -> Self {
        Self {
            root: "json5".into(),
            ..Default::default()
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn path_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.root);

        for v in &self.path {
            match v {
                JsonPath::Array(n) => write!(&mut s, "[{n}]").unwrap(),
                JsonPath::Object(k) => write!(&mut s, ".{k}").unwrap(),
            };
        }

        s
    }
}
