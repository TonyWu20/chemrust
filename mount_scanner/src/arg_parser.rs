use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
pub struct Args {
    pub file: String,
    pub radius: f64,
    #[arg(short = 'n', long)]
    pub dryrun: bool,
    #[arg(short, long)]
    pub output_name: Option<String>,
}
