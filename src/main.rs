mod cli;
mod input;

use clap::Parser;
use cli::{ArgInputType, Cli};
use std::io::{prelude::*, BufReader};
use termcolor::BufferWriter;

pub struct GronArgs {
    extension: Option<String>,
    input_type: ArgInputType,
    input: Box<dyn Read>,
    color_choice: termcolor::ColorChoice,
}

fn gron(args: GronArgs) -> anyhow::Result<()> {
    let mut reader = BufReader::new(args.input);

    let mut s = String::new();
    reader.read_to_string(&mut s)?;

    let writer = BufferWriter::stdout(args.color_choice);

    match args.input_type {
        ArgInputType::Json | ArgInputType::Json5 | ArgInputType::Yaml | ArgInputType::Toml => {
            writeln!(writer.buffer(), "{}", input::InputType::Toml.to_gron(&s)?)?
        }
        ArgInputType::Auto => match args
            .extension
            .and_then(input::InputType::guess_by_extension)
        {
            Some(ext) => {
                writeln!(writer.buffer(), "{}", ext.to_gron(&s)?)?;
            }
            None => writeln!(
                writer.buffer(),
                "{}",
                input::InputType::guess_and_to_gron(&s).unwrap()
            )?,
        },
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let args = GronArgs {
        extension: cli.input.extension(),
        input_type: cli.input_type,
        input: cli.input.read()?,
        color_choice: cli.color.color_choice(),
    };

    gron(args)
}
