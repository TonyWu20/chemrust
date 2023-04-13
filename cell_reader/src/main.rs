use std::{
    error::Error,
    fs::{self, read_to_string},
    path::Path,
};

use chemrust_formats::{Msi, StructureFile};
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
    if cli.dryrun {
        println!("{}", cell_model);
    } else {
        let msi_file: StructureFile<Msi> = StructureFile::new(cell_model);
        if let Some(output_name) = cli.output_name {
            fs::write(output_name, msi_file.export_msi())?;
        } else {
            let output_name = format!(
                "{}.msi",
                cell_filepath.file_stem().unwrap().to_str().unwrap()
            );
            fs::write(output_name, msi_file.export_msi())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use chemrust_formats::{Msi, StructureFile};
    use chemrust_parser::CellParser;

    #[test]
    fn test_write_msi() {
        let cell_filename = "../chemrust-parser/SAC_GDY_V.cell";
        let cell_file_text = read_to_string(cell_filename).unwrap();
        let cell_model = CellParser::new(&cell_file_text)
            .to_lattice_cart()
            .to_positions()
            .build_lattice();
        let msi_file: StructureFile<Msi> = StructureFile::new(cell_model);
        println!("{}", msi_file.export_msi());
    }
}
