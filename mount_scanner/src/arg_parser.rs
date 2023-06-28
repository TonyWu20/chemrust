use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
pub struct Args {
    pub file: String,
    pub radius: f64,
    pub element: String,
    #[arg(short, long)]
    pub output_name: Option<String>,
    #[arg(long)]
    pub potentials_loc: Option<String>,
    #[arg(short, long)]
    pub mode: Option<Mode>,
    #[arg(long)]
    pub edft: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Generate seed files without copying potentials
    Fast,
    /// Generate seed files and copy potentials
    Full,
    /// Copy potentials after seed files generation
    Post,
    /// Reorganize
    Dryrun,
    /// Debug
    Debug,
    /// Clean the generated folder
    Clean,
}
