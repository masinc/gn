mod cli;
mod input;

use clap::Parser;
use cli::{ArgInputType, Args};
use std::{
    fs,
    io::{self, prelude::*, BufReader},
};

fn load_input(input: &cli::Input) -> io::Result<Box<dyn Read>> {
    dbg!(input);
    match input {
        cli::Input::Stdin => Ok(Box::new(io::stdin().lock())),
        cli::Input::Path(s) => Ok(Box::new(fs::File::open(s)?)),
        cli::Input::Url(_) => todo!(),
    }
}

fn gron(input_type: ArgInputType, input: cli::Input) -> anyhow::Result<()> {
    let reader = load_input(&input);
    let mut reader = BufReader::new(reader?);

    let mut s = String::new();
    reader.read_to_string(&mut s)?;

    match input_type {
        ArgInputType::Json | ArgInputType::Json5 | ArgInputType::Yaml | ArgInputType::Toml => {
            println!("{}", input::InputType::Toml.to_gron(&s)?)
        }
        ArgInputType::Auto => {
            match input
                .extension()
                .and_then(input::InputType::guess_by_extension)
            {
                Some(ext) => {
                    println!("{}", ext.to_gron(&s)?);
                }
                None => println!("{}", input::InputType::guess_and_to_gron(&s).unwrap()),
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    gron(args.input_type, args.input)
}
