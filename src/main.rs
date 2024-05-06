use clap::Parser;
use core::str;
use serde_json::Value;
use std::{fs, path::Path};

mod ada;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, default_value = "options.json")]
    options_file: String,

    #[arg(short, long)]
    test: bool,
}

struct PathXEntry {
    dir: String,
    pathx: ada::PathX,
    idx: usize,
}

impl PathXEntry {
    fn new(path: &str, idx: usize) -> PathXEntry {
        PathXEntry {
            dir: Path::new(path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            pathx: ada::PathX::new(path),
            idx,
        }
    }
}

struct InputItem {
    path: String,
    group_cache: Vec<String>,
}

struct InputEntry {
    pathx_entry: PathXEntry,
    items: Vec<InputItem>,
}

fn main() {
    let args = Args::parse();

    // check if the options file exists
    if !fs::metadata(&args.options_file).is_ok() {
        println!("Error reading options file: {}", args.options_file);
        // print usage
        return;
    }

    // Read the file
    let op_file_json: Value =
        serde_json::from_reader(fs::File::open(&args.options_file).unwrap()).unwrap();
    let mut arg_array = op_file_json
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap());

    let mut options: Vec<String> = Vec::new();
    let mut output_path_option: Option<String> = None;
    let mut output_pathx_option: Option<PathXEntry> = None;
    let mut input_entries: Vec<InputEntry> = Vec::new();
    while let Some(mut arg) = arg_array.next() {
        match arg {
            "--output" => {
                options.push(arg.to_string());
                arg = arg_array.next().unwrap();
                output_path_option = Some(arg.to_string());
                output_pathx_option = Some(PathXEntry::new(arg, options.len()));
            }
            "(" => {
                options.push(arg.to_string());
                arg = arg_array.next().unwrap();
                input_entries.push(InputEntry {
                    pathx_entry: PathXEntry::new(arg, options.len()),
                    items: Vec::new(),
                });
            }
            _ => (),
        }
        options.push(arg.to_string());
    }

    let Some(output_path) = output_path_option else {
        println!("Error: output path not found in options file");
        return;
    };
    let Some(output_pathx_entry) = output_pathx_option else {
        println!("Error: output path not found in options file");
        return;
    };

    for input_entry in input_entries.iter_mut() {
        let Ok(dir_items) = fs::read_dir(&input_entry.pathx_entry.dir) else {
            println!("Error reading directory: {}", input_entry.pathx_entry.dir);
            return;
        };

        for dir_item in dir_items {
            let Ok(dir_item) = dir_item else {
                println!("Error reading directory entry");
                return;
            };

            let Some(item_groups) = input_entry
                .pathx_entry
                .pathx
                .get_groups(dir_item.path().to_str().unwrap())
            else {
                continue;
            };

            input_entry.items.push(InputItem {
                path: dir_item.path().to_str().unwrap().to_string(),
                group_cache: item_groups,
            });
        }
    }

    // check all input entries have the same number of items
    let num_items = input_entries[0].items.len();
    for input_entry in input_entries.iter().skip(1) {
        if input_entry.items.len() != num_items {
            println!("Error: input entries have different number of items");
            return;
        }
    }

    // check all input entries have the same groups
    let entry_groups_colletion: Vec<&Vec<String>> = input_entries[0]
        .items
        .iter()
        .map(|i| &i.group_cache)
        .collect();
    for input_entry in input_entries.iter().skip(1) {
        if input_entry
            .items
            .iter()
            .map(|i| &i.group_cache)
            .collect::<Vec<&Vec<String>>>()
            != entry_groups_colletion
        {
            println!("Error: input entries have different groups");
            return;
        }
    }

    for i in 0..num_items {
        options[output_pathx_entry.idx] = output_pathx_entry
            .pathx
            .get_path(&output_path, entry_groups_colletion[i])
            .unwrap();
        for input_entry in input_entries.iter() {
            options[input_entry.pathx_entry.idx] = input_entry
                .pathx_entry
                .pathx
                .get_path(&input_entry.items[i].path, entry_groups_colletion[i])
                .unwrap();
        }

        if args.test {
            println!("options: for group {}", entry_groups_colletion[i].join(","));
            for opt in options.iter() {
                println!("{}", opt);
            }
            println!("----------------");
        } else {
            println!("Merging for group {}", entry_groups_colletion[i].join(","));
            let cmd = std::process::Command::new("mkvmerge")
                .args(&options)
                .output()
                .expect("failed to execute process");
            println!("{}", str::from_utf8(&cmd.stdout).unwrap());
            println!("{}", str::from_utf8(&cmd.stderr).unwrap());
            println!("----------------")
        }
    }
}
