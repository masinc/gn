use crate::output::{
    json::{ColorSet, Context, JsonPath},
    OutputWriter, WriteColorExt,
};

use std::io;
use termcolor::WriteColor;

#[derive(Debug)]
pub struct Original {
    color_set: ColorSet,
}

impl Default for Original {
    fn default() -> Self {
        Self::new()
    }
}

impl Original {
    pub fn new() -> Self {
        Self {
            color_set: Default::default(),
        }
    }

    fn write_path(&self, mut writer: &mut dyn WriteColor, ctx: &Context) -> io::Result<()> {
        writer.write_color(&self.color_set.ns, &ctx.root)?;
        for p in &ctx.path {
            match p {
                JsonPath::Array(n) => {
                    writer.write_color(&self.color_set.bracket, "[")?;
                    writer.write_color(&self.color_set.number, &n.to_string())?;
                    writer.write_color(&self.color_set.bracket, "]")?;
                }
                JsonPath::Object(k) => {
                    write!(writer, ".")?;
                    writer.write_color(&self.color_set.ns, k)?;
                }
            }
        }

        write!(writer, " = ")
    }

    fn write_null(&self, mut writer: &mut dyn WriteColor, ctx: &Context) -> io::Result<()> {
        self.write_path(writer, ctx)?;
        writer.writeln_color(&self.color_set.null, "null")
    }

    fn write_bool(
        &self,
        mut writer: &mut dyn WriteColor,
        ctx: &Context,
        value: &bool,
    ) -> io::Result<()> {
        self.write_path(writer, ctx)?;
        writer.writeln_color(&self.color_set.bool, &value.to_string())
    }

    fn write_number(
        &self,
        mut writer: &mut dyn WriteColor,
        ctx: &Context,
        value: &serde_json::Number,
    ) -> io::Result<()> {
        self.write_path(writer, ctx)?;
        writer.writeln_color(&self.color_set.number, &value.to_string())
    }

    fn write_string(
        &self,
        mut writer: &mut dyn WriteColor,
        ctx: &Context,
        value: &str,
    ) -> io::Result<()> {
        self.write_path(writer, ctx)?;
        writer.writeln_color(&self.color_set.string, &format!("\"{value}\""))
    }

    fn write_array(&self, mut writer: &mut dyn WriteColor, ctx: &Context) -> io::Result<()> {
        self.write_path(writer, ctx)?;
        writer.writeln_color(&self.color_set.bracket, "[]")
    }

    fn write_object(&self, mut writer: &mut dyn WriteColor, ctx: &Context) -> io::Result<()> {
        self.write_path(writer, ctx)?;
        writer.writeln_color(&self.color_set.bracket, "{}")
    }
}

impl OutputWriter<serde_json::Value> for Original {
    type Error = io::Error;
    type Context = Context;

    fn init_ctx(&self) -> Self::Context {
        Default::default()
    }

    fn write_output(
        &mut self,
        writer: &mut dyn WriteColor,
        value: &serde_json::Value,
        ctx: &mut Self::Context,
        config: &crate::output::Config,
    ) -> Result<(), Self::Error> {
        match value {
            serde_json::Value::Null => self.write_null(writer, ctx)?,
            serde_json::Value::Bool(b) => self.write_bool(writer, ctx, b)?,
            serde_json::Value::Number(n) => self.write_number(writer, ctx, n)?,
            serde_json::Value::String(s) => self.write_string(writer, ctx, s)?,
            serde_json::Value::Array(a) => {
                self.write_array(writer, ctx)?;
                for (i, x) in a.iter().enumerate() {
                    ctx.path.push(JsonPath::Array(i));
                    self.write_output(writer, x, ctx, config)?;
                    ctx.path.pop();
                }
            }
            serde_json::Value::Object(o) => {
                self.write_object(writer, ctx)?;

                for (k, v) in o.iter() {
                    ctx.path.push(JsonPath::Object(k.to_string()));
                    self.write_output(writer, v, ctx, config)?;
                    ctx.path.pop();
                }
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::output::Config;

    use super::*;

    fn new_no_color() -> termcolor::NoColor<Vec<u8>> {
        termcolor::NoColor::new(vec![])
    }

    #[test]
    fn original_write_path() -> anyhow::Result<()> {
        let org = Original::new();
        let mut ctx = Context::new();

        // json
        let mut w = new_no_color();
        org.write_path(&mut w, &ctx)?;
        assert_eq!("json = ", String::from_utf8(w.into_inner())?);

        // json[0]
        ctx.path.push(JsonPath::Array(0));
        let mut w = new_no_color();
        org.write_path(&mut w, &ctx)?;
        assert_eq!("json[0] = ", String::from_utf8(w.into_inner())?);

        ctx.path.clear();

        // json.aaa
        ctx.path.push(JsonPath::Object("aaa".into()));
        let mut w = new_no_color();
        org.write_path(&mut w, &ctx)?;
        assert_eq!("json.aaa = ", String::from_utf8(w.into_inner())?);

        // json.aaa.bbb
        ctx.path.push(JsonPath::Object("bbb".into()));
        let mut w = new_no_color();
        org.write_path(&mut w, &ctx)?;
        assert_eq!("json.aaa.bbb = ", String::from_utf8(w.into_inner())?);

        // json.aaa.bbb[1]
        ctx.path.push(JsonPath::Array(1));
        let mut w = new_no_color();
        org.write_path(&mut w, &ctx)?;
        assert_eq!("json.aaa.bbb[1] = ", String::from_utf8(w.into_inner())?);

        Ok(())
    }

    #[test]
    fn original_write_null() -> anyhow::Result<()> {
        let org = Original::new();
        let ctx = Context::new();

        let mut w = new_no_color();
        org.write_null(&mut w, &ctx)?;
        assert_eq!("json = null\n", String::from_utf8(w.into_inner())?);

        Ok(())
    }

    #[test]
    fn original_write_bool() -> anyhow::Result<()> {
        let org = Original::new();
        let ctx = Context::new();

        let mut w = new_no_color();
        org.write_bool(&mut w, &ctx, &true)?;
        assert_eq!("json = true\n", String::from_utf8(w.into_inner())?);

        let mut w = new_no_color();
        org.write_bool(&mut w, &ctx, &false)?;
        assert_eq!("json = false\n", String::from_utf8(w.into_inner())?);

        Ok(())
    }

    #[test]
    fn original_write_number() -> anyhow::Result<()> {
        let org = Original::new();
        let ctx = Context::new();

        let mut w = new_no_color();
        org.write_number(&mut w, &ctx, &1.into())?;
        assert_eq!("json = 1\n", String::from_utf8(w.into_inner())?);

        let mut w = new_no_color();
        org.write_number(&mut w, &ctx, &(-1).into())?;
        assert_eq!("json = -1\n", String::from_utf8(w.into_inner())?);

        Ok(())
    }

    #[test]
    fn original_write_string() -> anyhow::Result<()> {
        let org = Original::new();
        let ctx = Context::new();

        let mut w = new_no_color();
        org.write_string(&mut w, &ctx, "")?;
        assert_eq!("json = \"\"\n", String::from_utf8(w.into_inner())?);

        let mut w = new_no_color();
        org.write_string(&mut w, &ctx, "1")?;
        assert_eq!("json = \"1\"\n", String::from_utf8(w.into_inner())?);

        Ok(())
    }

    #[test]
    fn original_write_array() -> anyhow::Result<()> {
        let org = Original::new();
        let ctx = Context::new();

        let mut w = new_no_color();
        org.write_array(&mut w, &ctx)?;
        assert_eq!("json = []\n", String::from_utf8(w.into_inner())?);

        Ok(())
    }

    #[test]
    fn original_write_object() -> anyhow::Result<()> {
        let org = Original::new();
        let ctx = Context::new();

        let mut w = new_no_color();
        org.write_object(&mut w, &ctx)?;
        assert_eq!("json = {}\n", String::from_utf8(w.into_inner())?);

        Ok(())
    }

    #[test]
    fn original_write_output() -> anyhow::Result<()> {
        let mut org = Original::new();

        let mut w = new_no_color();

        let v = serde_json::json!(
            {
                "a": [1, 2, 3],
                "b": true,
                "n": 1,
                "o": {
                    "b": false
                },
                "s": "1",
            }
        );
        org.write_output(&mut w, &v, &mut org.init_ctx(), &Config {})?;

        assert_eq!(
            r#"json = {}
json.a = []
json.a[0] = 1
json.a[1] = 2
json.a[2] = 3
json.b = true
json.n = 1
json.o = {}
json.o.b = false
json.s = "1"
"#,
            String::from_utf8(w.into_inner())?
        );

        Ok(())
    }
}
