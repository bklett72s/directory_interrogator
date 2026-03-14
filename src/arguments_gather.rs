use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Directory for scanning
    #[arg(short, long, default_value = "None")]
    pub directory: String,

    // Directory for output file
    #[arg(short, long, default_value = "None")]
    pub output: String,
}