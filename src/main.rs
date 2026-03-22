// Crates
use clap::Parser;
use serde::Deserialize;
use include_dir::{include_dir, Dir};
use csv::ReaderBuilder;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PySequence, PyString};
use pyo3_ffi::c_str;
use std::fs;

// Files in directory
mod file_dialog;
mod arguments_gather;
mod archive_interrogater;
mod folder_walk;
mod file_type_eval;

// Static directories
//static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data");
//static SCRIPTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/python_scripts");

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
struct mbits {
    Name: String,
    Signature: Option<String>,
    extension: Option<String>
}

// Function to detect if the program is running in an environment with a UI (Windows or Wayland)
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

// Reads the magic bits from the XML national archies file, and returns a vector of mbits structs
fn read_mbit_file() -> PyResult<Vec<mbits>> {

    Python::attach(|py: Python<'_>| {
        let python_script: &std::ffi::CStr = c_str!(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_scripts/magic_bit_scraper.py")));
        let from_python = Python::attach(|py| -> PyResult<Py<PyAny>> {
            let app : Py<PyAny> = PyModule::from_code(py, &python_script, c"magic_bit_scraper.py",c"")?
                .getattr("extract_magic_bit_data")?
                .into();
            app.call0(py)
        })?;
        
        let py_any = from_python.bind(py);


        let py_list = py_any.cast::<PyList>()?;

        
        let rust_vec: Vec<mbits> = py_list
            .iter()
            .map(|item| {
                let dict = item.cast::<PyDict>().unwrap();
                mbits {
                    Name: dict.get_item("Name").unwrap().unwrap().extract().unwrap(),
                    Signature: dict.get_item("Signature").unwrap().unwrap().extract().unwrap(),
                    extension: dict.get_item("extension").unwrap().unwrap().extract().unwrap(),
                }
            })
            .collect();

        Ok(rust_vec)
    })
}

// Main function, gathers arguments, detects UI, reads magic bits, walks target directory, evaluates file types
fn main() {
    println!("Program start"); // Delete me during prod

    let args: arguments_gather::Args = arguments_gather::Args::parse();
    let mut tgt_dir: String                 = String::new();
    let mut out_dir: String                 = String::new();
    let mut object_paths: Vec<String>       = Vec::new();
    let mut f_bits_vec: Vec<String>         = Vec::new();

    read_mbit_file().unwrap_or_else(|err| { // delete me post test
        eprintln!("Error reading mbit file: {}", err);
        std::process::exit(1);
    });

    let mbits_key: Vec<mbits> = read_mbit_file().unwrap_or_else(|err| {
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

    // Evaluate for filetypes START HERE 3/13/2026
    // Evaluate for zip files

    for path in object_paths {
        let f_bits = file_type_eval::file_bridge(&path, mbits_key.clone()).unwrap_or_else(|err| {
            eprintln!("Error during file type evaluation: {}... Path: {}", err, path);
            std::process::exit(1);
        });

        f_bits_vec.push(f_bits);

        for f_bit in &f_bits_vec {
            println!("File: {}, Magic Bits: {}", path, f_bit);
        }
    }
}
