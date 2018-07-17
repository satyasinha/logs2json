extern crate docopt;

use std::io::{self, BufRead};
use std::io::Error;
use docopt::Docopt;

// Write the Docopt usage string.
const USAGE: &'static str = "
logs2json is a mongod server log parser that coerces individual lines of server log to json
and pretty prints into one of the specified forms i.e. json or table.

Usage:
  logs2json (-j | --json)
  logs2json (-t | --table)
  logs2json (-h | --help)

Options:
  -h --help   show this help screen.
  -j --json   pretty print to json format.
  -t --table  pretty print to tabular format.
";

const JSON: &'static str = "json";
const TABLE: &'static str = "table";

fn main() {
  let args = Docopt::new(USAGE)
                    .and_then(|dopt | dopt.parse())
                    .unwrap_or_else(|e| e.exit());

  let mode = parse_args(&args);

  let stdin = io::stdin();

  for line in stdin.lock().lines() {
    pretty_print(line, mode);
  }
}

fn parse_args(args: &docopt::ArgvMap) -> &'static str {
  let mut parsed = JSON;

  if args.get_bool("-j") {
    parsed = JSON;
  } else if args.get_bool("-t") {
    parsed = TABLE;
  }

  return parsed;
}

fn pretty_print(line: Result<String, Error>, mode: &str) {
  println!("{}", line.unwrap());
  println!("{}", mode);
}
