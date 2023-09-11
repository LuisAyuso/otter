use clap::{value_parser, Arg, Command};

mod model;
mod parser;
mod skills;
mod solver;

fn main() {
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

    let _x: &String = matches.get_one("input").unwrap();
}
