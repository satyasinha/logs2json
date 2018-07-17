# logs2json
Skunkworks Q2 2018 project to parse mongodb server logs as json and pretty print.

## Description

This is a quick and dirty log parser written in rust. It can take JSON-ish logs and coerce them into strict JSON.

## Install

As this is a rust project running and building an executable is easy!

Follow these simple steps:
- If you haven't already please download and install rust from [here](https://www.rust-lang.org/en-US/install.html)
- Clone this repo via `git clone https://github.com/satyasinha/logs2json.git`
- Go into `logs2json` directory and run `cargo build` to build an executable for testing purposes, which should be created under `target/debug/`
- To build a release version try `cargo build --release` which produce a release build.
- To run it simply run `logs2json` from either either debug or target with a --help flag, the instruction should be fairly explanatory after that.

## Examples

### Help/Instructions
```
▶ ./logs2json --help
logs2json is a mongod server log parser that coerces individual lines of server log to json
and pretty prints into one of the specified forms i.e. json or table. It can also print in its
original messy form with --original/-o flag.

Usage:
  logs2json (-j | --json)
  logs2json (-t | --table)
  logs2json (-o | --original)
  logs2json (-h | --help)

Options:
  -h --help       show this help screen.
  -j --json       pretty print to json format.
  -t --table      pretty print to tabular format.
  -o --original   original raw messy format.

```
### Sample log run
```
▶ target/debug/logs2json --json < ../sample.log 
{
  "base": "fd limit hard:1048576 soft:1024 max conn: 819",
  "c": "network",
  "ctx": "main",
  "msg": [],
  "s": "debug",
  "t": {
    "$date": "2018-07-15T06:36:53.947+0000"
  }
}
...
...
```
