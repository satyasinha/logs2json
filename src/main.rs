extern crate docopt;
extern crate serde_json;

use std::io::{self, BufRead};
use docopt::Docopt;
use serde_json::Value;

// Write the Docopt usage string.
const USAGE: &'static str = "
logs2json is a mongod server log parser that coerces individual lines of server log to json
and pretty prints into one of the specified forms i.e. json or table or plain server format.
It can also print in its original messy form with --original/-o flag.

Usage:
  logs2json (-j | --json)
  logs2json (-t | --table)
  logs2json (-o | --original)
  logs2json (-h | --help)
  logs2json (-p | --plain)

Options:
  -h --help       show this help screen.
  -j --json       pretty print to json format.
  -t --table      pretty print to tabular format.
  -o --original   original raw messy format.
  -p --plain      plain old server log format.
";

// docopts options
const JSON: &'static str = "json";
const TABLE: &'static str = "table";
const RAW: &'static str = "raw";
const PLAIN: &'static str = "plain";

// table column min lengths
const C_MIN_LENGTH: usize = 10;
const CTX_MIN_LENGTH: usize = 28;
const MSG_MIN_LENGTH: usize = 6;
const S_MIN_LENGTH: usize = 7;
const T_MIN_LENGTH: usize = 30;
const BASE_MIN_LENGTH: usize = 45;
const BAR: &'static str = "----------------------------------------------";

// severity types
const INFO: &'static str = "info";
const DEBUG: &'static str = "debug";
const SEVERE: &'static str = "severe";
const WARNING: &'static str = "warning";
const ERROR: &'static str = "error";

// component types
const kDefault: &'static str = "default";
const kAccessControl: &'static str = "accessControl";
const kCommand: &'static str = "command";
const kControl: &'static str = "control";
const kExecutor: &'static str = "executor";
const kGeo: &'static str = "geo";
const kIndex: &'static str = "index";
const kNetwork: &'static str = "network";
const kQuery: &'static str = "query";
const kReplication: &'static str = "replication";
const kReplicationHeartbeats: &'static str = "heartbeats";
const kReplicationRollback: &'static str = "rollback";
const kSharding: &'static str = "sharding";
const kStorage: &'static str = "storage";
const kStorageRecovery: &'static str = "recovery";
const kJournal: &'static str = "journal";
const kWrite: &'static str = "write";
const kFTDC: &'static str = "ftdc";
const kASIO: &'static str = "asio";
const kBridge: &'static str = "bridge";
const kTracking: &'static str = "tracking";
const kNumLogComponents: &'static str = "total";

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
  } else if args.get_bool("-o") {
    parsed = RAW;
  } else if args.get_bool("-p") {
    parsed = PLAIN;
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
  } else if mode == RAW {
    println!("{}", serde_json::to_string(&json_line).unwrap());
  } else  if mode == TABLE {
    let c = serde_json::to_string(&json_line["c"]).unwrap();
    let ctx = serde_json::to_string(&json_line["ctx"]).unwrap();
    let msg = serde_json::to_string(&json_line["msg"]).unwrap();
    let s = serde_json::to_string(&json_line["s"]).unwrap();
    let t = json_line["t"]["$date"].as_str().unwrap();
    let base = serde_json::to_string(&json_line["base"]).unwrap();

    println!("{0:1$} | {2:3$} | {4:5$} | {6:7$} | {8:9$} | {10:11$}",
      c, C_MIN_LENGTH, ctx, CTX_MIN_LENGTH, s, S_MIN_LENGTH,
      t, T_MIN_LENGTH, msg, MSG_MIN_LENGTH, base, BASE_MIN_LENGTH);
  }  else if mode == PLAIN {
    let t = json_line["t"]["$date"].as_str().unwrap();
    let severity = map_severity(json_line["s"].as_str().unwrap());
    let component = if json_line["c"].is_string() {
      map_component(json_line["c"].as_str().unwrap())
    } else {
      "-"
    };

    let context = json_line["ctx"].as_str().unwrap();
    let mut message: String = "".to_string();

    for msg in json_line["msg"].as_array().unwrap() {
      let mut pretty_msg: &str  = "";

      if msg.is_string() {
        message.push_str(msg.as_str().unwrap());
      } else {
        message.push_str(&serde_json::to_string(msg).unwrap());
      };
    }

    // TODO: lengths are hard-coded here, very naughty
    println!("{} {} {:8} [{}] {}", t, severity, component, context, message);
  }
}

// appropriated from: https://github.com/devkev/mongo/blob/slog6/src/mongo/logger/log_severity.cpp#L62
fn map_severity(severity: &str) -> &'static str {
  if severity == INFO {
    return "I";
  } else if severity == DEBUG {
    return "D";
  } else if severity == WARNING {
    return "W";
  } else if severity == SEVERE {
    return "F";
  } else if severity == ERROR {
    return "E";
  }

  return "U";
}

// appropriated from: https://github.com/devkev/mongo/blob/slog6/src/mongo/logger/log_component.cpp
fn map_component(component: &str) -> &'static str {
  if component == kDefault {
    return "-";
  } else if component == kAccessControl {
    return "ACCESS";
  } else if component == kCommand {
    return "COMMAND";
  } else if component == kControl {
    return "CONTROL";
  } else if component == kExecutor {
    return "EXECUTOR";
  } else if component == kGeo {
    return "GEO";
  } else if component == kIndex {
    return "INDEX";
  } else if component == kNetwork {
    return "NETWORK";
  } else if component == kQuery {
    return "QUERY";
  } else if component == kReplication {
    return "REPL";
  } else if component == kReplicationHeartbeats {
    return "REPL_HB";
  } else if component == kReplicationRollback {
    return "ROLLBACK";
  } else if component == kSharding {
    return "SHARDING";
  } else if component == kStorage {
    return "STORAGE";
  } else if component == kStorageRecovery {
    return "RECOVERY";
  } else if component == kJournal {
    return "JOURNAL";
  } else if component == kWrite {
    return "WRITE";
  } else if component == kFTDC {
    return "FTDC";
  } else if component == kASIO {
    return "ASIO";
  } else if component == kBridge {
    return "BRIDGE";
  } else if component == kTracking {
    return "TRACKING";
  } else if component == kNumLogComponents {
    return "TOTAL";
  }

  return "U";
}
