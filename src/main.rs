use clap::{Command, Parser};
use std::fs;

use serde_json::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg(short, long, default_value = ".")]
    // target_dir: String,

    // #[arg(short, long, default_value = "./output")]
    // output_dir: String,
    #[arg(short = 'i', long, default_value = "output.json")]
    options_file: String,
}

fn main() {
    let args = Args::parse();
    // let mut op_file_paths = Vec::new();

    let mut output_path: Option<String> = None;

    // let mut mkv_file_paths = Vec::new();

    // let Ok(dir_entries) = fs::read_dir(&args.target_dir) else {
    //     eprintln!("Error reading directory: {}", args.target_dir);
    //     return;
    // };

    // print!("Scanning directory: {}", &args.target_dir);
    // for entry in dir_entries {
    //     let Ok(entry) = entry else {
    //         eprintln!("Error reading directory entry {:?}", entry);
    //         continue;
    //     };
    //     let path = entry.path();
    //     if !path.is_file() {
    //         println!("Skipping non-file: {}", path.display());
    //         continue;
    //     }
    //     let path_ext = path.extension().and_then(|s| s.to_str());
    //     match path_ext {
    //         Some("mkv") => {
    //             println!("Found MKV file: {}", path.display());
    //             mkv_file_paths.push(path);
    //         }
    //         Some("json") => {
    //             println!("Found JSON file: {}", path.display());
    //             op_file_paths.push(path);
    //         }
    //         _ => {
    //             println!("Skipping unknown file: {}", path.display());
    //             continue;
    //         }
    //     }
    // }

    // println!("Found {} option files", op_file_paths.len());
    // println!("Found {} MKV files", mkv_file_paths.len());

    // println!("\nProcessing option files");

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

    // let mut arg_array_iter = arg_array.into_iter();
    // for arg in arg_array_iter {
    //     if arg == "--output" {
    //         output_path = Some(arg_array_iter.next().unwrap().to_string());
    //     }
    // }

    while let Some(arg) = arg_array.next() {
        if arg == "--output" {
            output_path = Some(arg_array.next().unwrap().to_string());
        }
    }

    println!("Output path: {:?}", output_path);
}
