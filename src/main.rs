mod cli;
mod input;
mod output;

use crate::output::{Config, WriterKind};
use anyhow::bail;
use clap::Parser;
use cli::{ArgInputType, Cli};
use std::io::{self, prelude::*, BufReader};

pub struct GronArgs {
    extension: Option<String>,
    input_type: ArgInputType,
    input: Box<dyn Read>,
    output_type: WriterKind,
    writer: Box<dyn Write>,
    color: output::Color,
}

fn gron_auto(args: &mut GronArgs) -> anyhow::Result<()> {
    use input::InputType;
    match args
        .extension
        .as_ref()
        .and_then(InputType::guess_by_extension)
    {
        Some(guessed_type) => {
            let mut args = args;
            args.input_type = ArgInputType::from(guessed_type);
            gron(args)?;
        }
        None => {
            let mut reader = BufReader::new(&mut args.input);

            let mut s = String::new();
            reader.read_to_string(&mut s)?;

            if (serde_json::from_str(&s) as Result<serde_json::Value, _>).is_ok() {
                InputType::Json.write(
                    &mut args.writer,
                    &s,
                    &args.output_type,
                    &Config { color: args.color },
                )?;
                return Ok(());
            }

            if (json5::from_str(&s) as Result<serde_json::Value, _>).is_ok() {
                InputType::Json5.write(
                    &mut args.writer,
                    &s,
                    &args.output_type,
                    &Config { color: args.color },
                )?;
                return Ok(());
            }

            if (toml::from_str(&s) as Result<toml::Value, _>).is_ok() {
                InputType::Toml.write(
                    &mut args.writer,
                    &s,
                    &args.output_type,
                    &Config { color: args.color },
                )?;
                return Ok(());
            }

            if (serde_yaml::from_str(&s) as Result<serde_json::Value, _>).is_ok() {
                InputType::Yaml.write(
                    &mut args.writer,
                    &s,
                    &args.output_type,
                    &Config { color: args.color },
                )?;
                return Ok(());
            }

            bail!("Input format auto detection failed.")
        }
    }

    Ok(())
}

fn gron(args: &mut GronArgs) -> anyhow::Result<()> {
    match input::InputType::try_from(args.input_type) {
        Ok(input_type) => {
            let mut reader = BufReader::new(&mut args.input);

            let mut s = String::new();
            reader.read_to_string(&mut s)?;

            input_type.write(
                &mut args.writer,
                &s,
                &args.output_type,
                &Config { color: args.color },
            )?;
        }
        // auto
        Err(_) => gron_auto(args)?,
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut args = GronArgs {
        extension: cli.input.extension(),
        input_type: cli.input_type,
        input: cli.input.read()?,
        output_type: WriterKind::Gron,
        color: cli.color.into(),
        writer: Box::new(io::stdout().lock()),
    };

    gron(&mut args)
}
