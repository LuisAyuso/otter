use std::any;

use anyhow::Context;
use clap::{value_parser, Arg, Command};
use parser::from_file;

mod model;
mod parser;
mod skills;
mod solver;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("MyApp")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .required(true)
                .takes_value(true)
                .value_parser(value_parser!(String))
                .help("the file with the pitch config"),
        )
        .get_matches(); // builds the instance of ArgMatches

    let x: &String = matches.get_one("input").context("no argument")?;
    from_file(std::path::Path::new(x.as_str()))?;
    Ok(())
}
