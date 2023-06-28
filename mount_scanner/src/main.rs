use std::fs::read_to_string;
use std::{error::Error, path::Path, process::Command};

use arg_parser::{Args, Mode};
use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

use chemrust_parser::CellParser;
use chemrust_scanner::MountingChecker;
use clap::Parser;
use export_res::{post_copy_potentials, ExportManager};

mod arg_parser;
mod export_res;

fn main() -> Result<(), Box<dyn Error>> {
    // CLI interpretation
    let cli = Args::parse();
    let cwd = env!("CARGO_MANIFEST_DIR");
    let radius = cli.radius;
    let edft = cli.edft.unwrap_or(false);
    let pot_loc = cli.potentials_loc.as_ref();
    let new_element = ELEMENT_TABLE.get_by_symbol(&cli.element).unwrap();
    let export_loc = format!("{}/../{}_{}_to_SAC_GDY", cwd, new_element.symbol(), radius);
    let potential_loc_path = match pot_loc {
        Some(dir) => format!("{}/../{}", cwd, dir),
        None => format!("{}/../Potentials", cwd),
    };
    let mode = cli.mode.as_ref();
    // Task start
    let mount_checker = MountingChecker::new_builder()
        .with_element(new_element)
        .with_bondlength(radius)
        .build();
    // Read from cell
    let cell_filename = cli.file;
    let cell_filepath = Path::new(&cell_filename);
    let cell_seedname = cell_filepath.file_stem().unwrap();
    cell_filepath.try_exists()?;
    let cell_file_text = read_to_string(cell_filepath)?;
    let cell_model = CellParser::new(&cell_file_text)
        .to_lattice_cart()
        .to_positions()
        .build_lattice();
    let final_report = mount_checker.mount_search(cell_model.atoms());
    match mode {
        Some(m) => match m {
            Mode::Debug => {
                println!("{}", cwd);
                println!("{}", potential_loc_path);
            }
            Mode::Fast => {
                let export_manager = ExportManager::new(
                    new_element.symbol(),
                    &export_loc,
                    &potential_loc_path,
                    edft,
                );
                export_manager.export_sphere_model(
                    &final_report,
                    &cell_model,
                    cell_seedname.to_str().unwrap(),
                )?;
                export_manager.export_circles_model(
                    &final_report,
                    &cell_model,
                    cell_seedname.to_str().unwrap(),
                )?;
                export_manager.export_points_model(
                    &final_report,
                    &cell_model,
                    cell_seedname.to_str().unwrap(),
                )?;
            }
            Mode::Dryrun => {
                println!("Scanning model: {}", cell_filename);
                println!(
                    "New element: {}, Target bondlength: {} Å",
                    new_element.symbol(),
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
            }
            Mode::Post => {
                post_copy_potentials(&export_loc, &potential_loc_path)?;
            }
            Mode::Clean => {
                let target_dir = format!("{}_to_SAC_GDY", new_element.symbol());
                if Path::new(&target_dir).exists() {
                    Command::new("rm")
                        .args(["-r", &target_dir])
                        .output()
                        .expect(&format!("Error while deleting '{}'", &target_dir));
                }
            }
            Mode::Full => {
                let export_manager = ExportManager::new(
                    new_element.symbol(),
                    &export_loc,
                    &potential_loc_path,
                    edft,
                );
                export_manager.export_sphere_model(
                    &final_report,
                    &cell_model,
                    cell_seedname.to_str().unwrap(),
                )?;
                export_manager.export_circles_model(
                    &final_report,
                    &cell_model,
                    cell_seedname.to_str().unwrap(),
                )?;
                export_manager.export_points_model(
                    &final_report,
                    &cell_model,
                    cell_seedname.to_str().unwrap(),
                )?;
                post_copy_potentials(&export_loc, &potential_loc_path)?;
            }
        },
        None => {
            let export_manager =
                ExportManager::new(new_element.symbol(), &export_loc, &potential_loc_path, edft);
            export_manager.export_sphere_model(
                &final_report,
                &cell_model,
                cell_seedname.to_str().unwrap(),
            )?;
            export_manager.export_circles_model(
                &final_report,
                &cell_model,
                cell_seedname.to_str().unwrap(),
            )?;
            export_manager.export_points_model(
                &final_report,
                &cell_model,
                cell_seedname.to_str().unwrap(),
            )?;
        }
    }
    Ok(())
}
