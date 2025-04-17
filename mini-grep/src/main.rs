use clap::{App, Arg};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufReader, prelude::*},
};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        // let pattern = re.as_str();

        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let input_arg = Arg::with_name("input")
        .help("File to search")
        .takes_value(true)
        .required(false);

    let pattern_arg = Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true);

    let args = App::new("mini-grep")
        .version("0.1")
        .about("searches for patterns")
        .arg(pattern_arg)
        .arg(input_arg)
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
