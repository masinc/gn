mod cli;
mod input;
mod output;

use crate::output::{Config, WriterKind};
use clap::Parser;
use cli::{ArgInputType, Cli};
use std::io::{prelude::*, BufReader};

pub struct GronArgs {
    extension: Option<String>,
    input_type: ArgInputType,
    input: Box<dyn Read>,
    output_type: WriterKind,
    color_choice: termcolor::ColorChoice,
}

fn gron(args: &mut GronArgs) -> anyhow::Result<()> {
    match input::InputType::try_from(args.input_type) {
        Ok(input_type) => {
            let mut writer = termcolor::StandardStream::stdout(args.color_choice);
            let mut reader = BufReader::new(&mut args.input);

            let mut s = String::new();
            reader.read_to_string(&mut s)?;

            input_type.write(&mut writer, &s, &args.output_type, &Config {})?;
        }
        // auto
        Err(_) => {
            match args
                .extension
                .as_ref()
                .and_then(input::InputType::guess_by_extension)
            {
                Some(guessed_type) => {
                    let mut args = args;
                    args.input_type = ArgInputType::from(guessed_type);
                    gron(args)?;
                }
                None => {
                    todo!()
                }
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut args = GronArgs {
        extension: cli.input.extension(),
        input_type: cli.input_type,
        input: cli.input.read()?,
        output_type: WriterKind::Original,
        color_choice: cli.color.color_choice(),
    };

    gron(&mut args)
}
