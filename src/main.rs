use clap::{builder::Str, Parser};
use regex::Regex;
use std::{fs, path};

use serde_json::Value;

mod ada;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, default_value = "output.json")]
    options_file: String,
}

fn main() {
    let args = Args::parse();

    let mut output_path: Option<ada::PathX> = None;

    // check if the options file exists
    if !fs::metadata(&args.options_file).is_ok() {
        println!("Error reading options file: {}", args.options_file);
        // print usage

        return;
    }

    // Read the file
    let op_file_json: Value =
        serde_json::from_str(&fs::read_to_string(&args.options_file).unwrap()).unwrap();
    // println!("Processing option file: {}", op_file_path.display());
    let mut arg_array = op_file_json
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap());

    while let Some(arg) = arg_array.next() {
        if arg == "--output" {
            output_path = Some(ada::PathX::new(arg_array.next().unwrap().to_string()));
        }
    }
}
