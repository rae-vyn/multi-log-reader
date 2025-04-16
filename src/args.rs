use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// The path to the file to read.
    pub file_path: String,

}