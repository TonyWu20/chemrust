use std::{error::Error, fs::read_to_string, path::Path};

use chemrust_parser::CellParser;
use clap::Parser;

use crate::arg_parser::Args;

mod arg_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    let cell_filename = cli.file;
    let cell_filepath = Path::new(&cell_filename);
    cell_filepath.try_exists()?;
    let cell_file_text = read_to_string(cell_filepath)?;
    let cell_model = CellParser::new(&cell_file_text)
        .to_lattice_cart()
        .to_positions()
        .build_lattice();
    if let Some(true) = cli.dryrun {
        println!("{}", cell_model);
    } else {
        println!("Hello, world!");
    }
    Ok(())
}
