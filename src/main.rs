extern crate docopt;

use std::io::{self, BufRead};
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

fn main() {
  let args = Docopt::new(USAGE)
                    .and_then(|dopt | dopt.parse())
                    .unwrap_or_else(|e| e.exit());

  println!("{:?}", args);

  let stdin = io::stdin();

  for line in stdin.lock().lines() {
    println!("{}", line.unwrap());
  }
}
