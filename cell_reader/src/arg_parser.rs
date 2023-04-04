use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
pub struct Args {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub dryrun: Option<bool>,
    #[arg(short, long)]
    pub output_name: Option<String>,
}
