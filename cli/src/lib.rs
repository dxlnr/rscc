use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The path to the .c file which should be compiled.
    #[clap(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

#[derive(Debug)]
pub struct FileError {
    // pub problem: SourceError<'a, T>,
    pub path: std::path::PathBuf,
}