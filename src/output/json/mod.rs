mod js;
mod org;

pub use js::JavaScript;
pub use org::Original;
use std::{fmt::Write, io};
use termcolor::{Color, ColorSpec, WriteColor};

pub type WriteError = io::Error;
pub type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug, Clone)]
pub enum JsonPath {
    /// array index
    Array(usize),
    /// object key
    Object(String),
}

#[derive(Debug)]
pub struct Context {
    root: String,
    path: Vec<JsonPath>,
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

pub trait WriteJson {
    fn write_path(&self, writer: &mut dyn WriteColor, ctx: &Context) -> WriteResult<()>;
    fn write_null(&self, writer: &mut dyn WriteColor, ctx: &Context) -> WriteResult<()>;
    fn write_bool(
        &self,
        writer: &mut dyn WriteColor,
        ctx: &Context,
        value: &bool,
    ) -> WriteResult<()>;
    fn write_number(
        &self,
        writer: &mut dyn WriteColor,
        ctx: &Context,
        value: &serde_json::Number,
    ) -> WriteResult<()>;
    fn write_string(
        &self,
        writer: &mut dyn WriteColor,
        ctx: &Context,
        value: &str,
    ) -> WriteResult<()>;
    fn write_array(&self, writer: &mut dyn WriteColor, ctx: &Context) -> WriteResult<()>;
    fn write_object(&self, writer: &mut dyn WriteColor, ctx: &Context) -> WriteResult<()>;
}

#[derive(Debug)]
pub struct ColorSet {
    pub ns: ColorSpec,
    pub bracket: ColorSpec,

    pub number: ColorSpec,
    pub string: ColorSpec,
    pub bool: ColorSpec,
    pub null: ColorSpec,
}

impl ColorSet {
    pub fn new() -> Self {
        Self {
            ns: ColorSpec::new().set_fg(Some(Color::Blue)).clone(),
            bracket: ColorSpec::new().set_fg(Some(Color::Magenta)).clone(),

            number: ColorSpec::new().set_fg(Some(Color::Red)).clone(),
            string: ColorSpec::new().set_fg(Some(Color::Yellow)).clone(),
            bool: ColorSpec::new().set_fg(Some(Color::Cyan)).clone(),
            null: ColorSpec::new().set_fg(Some(Color::Cyan)).clone(),
        }
    }
}

impl Default for ColorSet {
    fn default() -> Self {
        Self::new()
    }
}
