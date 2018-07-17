extern crate docopt;
extern crate serde_json;

use std::io::{self, BufRead};
use docopt::Docopt;
use serde_json::Value;

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

const C_MIN_LENGTH: usize = 10;
const CTX_MIN_LENGTH: usize = 28;
const MSG_MIN_LENGTH: usize = 6;
const S_MIN_LENGTH: usize = 7;
const T_MIN_LENGTH: usize = 40;
const BASE_MIN_LENGTH: usize = 45;
const BAR: &'static str = "----------------------------------------------";

fn main() {
  let args = Docopt::new(USAGE)
                    .and_then(|dopt | dopt.parse())
                    .unwrap_or_else(|e| e.exit());

  let mode = parse_args(&args);

  let stdin = io::stdin();

  print_header(mode);

  for line in stdin.lock().lines() {
    pretty_print(&line.unwrap(), mode);
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

fn print_header(mode: &str) {
  if mode == TABLE {
    println!("{0:1$} | {2:3$} | {4:5$} | {6:7$} | {8:9$} | {10:11$}",
      "c", C_MIN_LENGTH, "ctx", CTX_MIN_LENGTH, "s", S_MIN_LENGTH,
      "t", T_MIN_LENGTH, "msg", MSG_MIN_LENGTH, "base", BASE_MIN_LENGTH);
    println!("{}{}{}{}", BAR, BAR, BAR, BAR);
  }
}

fn pretty_print(line: &String, mode: &str) {
   let json_line: Value = serde_json::from_str(line).unwrap();

  if mode == JSON {
   println!("{}", serde_json::to_string_pretty(&json_line).unwrap());
  } else if mode == TABLE {
    let c = serde_json::to_string(&json_line["c"]).unwrap();
    let ctx = serde_json::to_string(&json_line["ctx"]).unwrap();
    let msg = serde_json::to_string(&json_line["msg"]).unwrap();
    let s = serde_json::to_string(&json_line["s"]).unwrap();
    let t = serde_json::to_string(&json_line["t"]).unwrap();
    let base = serde_json::to_string(&json_line["base"]).unwrap();

    println!("{0:1$} | {2:3$} | {4:5$} | {6:7$} | {8:9$} | {10:11$}",
      c, C_MIN_LENGTH, ctx, CTX_MIN_LENGTH, s, S_MIN_LENGTH,
      t, T_MIN_LENGTH, msg, MSG_MIN_LENGTH, base, BASE_MIN_LENGTH);
  }
}
