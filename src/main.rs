// Crates
use clap::Parser;

// Files in directory
mod file_dialog;
mod arguments_gather;
mod archive_interrogater;
mod folder_walk;

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

fn main() {
    println!("Program start"); // Delete me during prod

    let args: arguments_gather::Args = arguments_gather::Args::parse();
    let mut tgt_dir: String              = String::new();
    let mut out_dir: String              = String::new();
    let mut object_paths: Vec<String>    = Vec::new();

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
    for i in object_paths {
        println!("Object Path: {}", i);
    }
}
