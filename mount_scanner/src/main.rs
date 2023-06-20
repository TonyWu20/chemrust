use std::{
    error::Error,
    fs::{self, read_to_string},
    path::Path,
};

use arg_parser::Args;
use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};
use chemrust_core::data::lattice::LatticeModel;
use chemrust_formats::{Cell, StructureFile};
use chemrust_parser::CellParser;
use chemrust_scanner::MountingChecker;
use clap::Parser;

mod arg_parser;
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    let radius = cli.radius;
    let element = ELEMENT_TABLE.get_by_symbol(&cli.element).unwrap();
    let mount_checker = MountingChecker::new_builder()
        .with_element(element)
        .with_bondlength(radius)
        .build();
    // Read from cell
    let cell_filename = cli.file;
    let cell_filepath = Path::new(&cell_filename);
    cell_filepath.try_exists()?;
    let cell_file_text = read_to_string(cell_filepath)?;
    let cell_model = CellParser::new(&cell_file_text)
        .to_lattice_cart()
        .to_positions()
        .build_lattice();
    let final_report = mount_checker.mount_search(cell_model.atoms());
    let visualize_atoms = final_report.visualize_atoms();
    let cell_model_atoms = cell_model.atoms().clone();
    let new_all_atoms = vec![cell_model_atoms, &visualize_atoms];
    let new_all_atoms = new_all_atoms.concat();
    let new_lattice_vectors = cell_model.lattice_vectors().unwrap().to_owned();
    let new_cell_model = LatticeModel::new(&Some(new_lattice_vectors), &new_all_atoms);
    let cell_output_file = StructureFile::<Cell>::new(new_cell_model);
    let lattice_vector_text = cell_output_file.write_lattice_vectors();
    let output_cell_atoms = cell_output_file.write_atoms();
    if cli.dryrun {
        println!("{}", output_cell_atoms);
    } else {
        if let Some(output_name) = cli.output_name {
            let contents = vec![lattice_vector_text, output_cell_atoms];
            let contents = contents.join("\n");
            fs::write(output_name, contents)?;
        } else {
            let output_name = format!(
                "{}.cell",
                cell_filepath.file_stem().unwrap().to_str().unwrap()
            );
            let contents = vec![lattice_vector_text, output_cell_atoms];
            let contents = contents.join("\n");
            fs::write(output_name, contents)?;
        }
    }
    println!("Scanning model: {}", cell_filename);
    println!(
        "New element: {}, Target bondlength: {} Å",
        element.symbol(),
        radius
    );
    let available_elements = mount_checker.available_elements(cell_model.atoms());
    println!("Available bonding elements:");
    available_elements.iter().for_each(|elm| println!("{elm}"));
    println!("---------------------------------");
    println!("Number of CN = 1: {}", final_report.sphere_sites().len());
    println!("Located as Spheres");
    // final_report.sphere_sites().iter().for_each(|p| {
    //     println!(
    //         "Sphere center: {}, around atom No.: {}",
    //         p.sphere().center,
    //         p.locating_atom_id()
    //     )
    // });
    println!("CN = 2: {}", final_report.circles().len());
    println!("As circles between two atoms");
    // final_report.circles().iter().for_each(|p| {
    //     println!(
    //         "Located at {}, between atom {} and {}",
    //         p.circle().center,
    //         p.connecting_atoms()[0],
    //         p.connecting_atoms()[1]
    //     )
    // });
    println!("Multi points: {}", final_report.multi_cn_points().len());
    final_report.multi_cn_points().iter().for_each(|p| {
        println!(
            "Coord: {:#}, Connecting: {:?}",
            p.coord(),
            p.connecting_atom_ids()
        )
    });
    Ok(())
}
