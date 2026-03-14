use rfd::FileDialog; // File Browser Window
use std::env;
use std::path::{PathBuf};

// File browser window for target directory
pub fn tgt_browser() -> String {
    let current_dir = env::current_dir().unwrap();
    let file = FileDialog::new()
        .set_title("Select Target Directory")
        .set_directory(&current_dir)
        .pick_folder();
    return file.unwrap_or(PathBuf::from("None")).to_str().unwrap().to_string();
}

// File browser window for output directory
pub fn out_browser() -> String {
    let current_dir = env::current_dir().unwrap();
    let file = FileDialog::new()
        .set_title("Select Output Directory")
        .set_directory(&current_dir)
        .pick_folder();
    return file.unwrap_or(PathBuf::from("None")).to_str().unwrap().to_string();
}