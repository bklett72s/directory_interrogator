// Crates
use clap::Parser;
use serde::Deserialize;
use include_dir::{include_dir, Dir};
use csv::ReaderBuilder;

// Files in directory
mod file_dialog;
mod arguments_gather;
mod archive_interrogater;
mod folder_walk;
mod file_type_eval;

// Static directories
static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data");

#[derive(Debug, Deserialize)]
struct mbits {
    extension: String,
    magic_number: String
}

fn has_ui() -> bool{
    // Detect Windows UI
    #[cfg(windows)]
    {
        atty::is(atty::Stream::Stdin)
    }

    // Detect Wayland UI
    #[cfg(unix)]
    {
        std::env::var("DISPLAY").is_ok()
            || std::env::var("WAYLAND_DISPLAY").is_ok()
    }
}

fn read_mbit_file() -> Result<Vec<mbits>, Box<dyn std::error::Error>> {
    // Placeholder for the actual mbit file reading logic
    let mbis_csv_file = ASSETS_DIR
        .get_file("file_signatures.csv")
        .expect("CSV file not found");

    let mbits_csv_data = mbis_csv_file.contents_utf8().expect("Invalid UTF-8 in CSV");

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(mbits_csv_data.as_bytes());

    let mut mbits_list: Vec<mbits> = Vec::new();

    let mbits_key: Vec<mbits> = rdr.deserialize()
        .map(|result| result.expect("Failed to parse record"))
        .collect();

    for mbit in &mbits_key {// Delete me during prod
        println!("Extension: {}, Magic Number: {}", mbit.extension, mbit.magic_number);
    }

    Ok(mbits_key)
}

fn main() {
    println!("Program start"); // Delete me during prod

    let args: arguments_gather::Args = arguments_gather::Args::parse();
    let mut tgt_dir: String              = String::new();
    let mut out_dir: String              = String::new();
    let mut object_paths: Vec<String>    = Vec::new();

    let mbits_key = read_mbit_file().unwrap_or_else(|err| {
        eprintln!("Error reading mbit file: {}", err);
        std::process::exit(1);
    });

    // If has UI, open file dialog windows
    if has_ui() {
        tgt_dir = file_dialog::tgt_browser();
        out_dir = file_dialog::out_browser();

        println!("Target Directory: {}", tgt_dir);
        println!("Output Directory: {}", out_dir);
    }
    else {
        tgt_dir = args.directory.clone();
        out_dir = args.output.clone();

        println!("No UI detected, using command line arguments.");
        println!("Target Directory: {}", tgt_dir);
        println!("Output Directory: {}", out_dir);
    }


    // Vector to store paths of objects detected
    object_paths = folder_walk::os_walk_bridge(&tgt_dir).unwrap_or_else(|err| {
        eprintln!("Error during folder walk: {}... Path: {}", err, tgt_dir);
        std::process::exit(1);
    });

    // Print object paths for testing, delete me during prod
    //for i in object_paths {
        //println!("Object Path: {}", i);
    //}

    // Evaluate for zip files
    //for path in object_paths {
        //let f_type = file_type_eval::file_type_eval(&path);
    //}
}
