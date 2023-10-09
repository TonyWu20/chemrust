use std::{error::Error, path::Path};

use clap::Parser;

use interactive_ui::RunOptions;

use crate::arg_parser::ProgramMode;
use crate::execution::{Config, Executor};
use crate::yaml_parser::TaskTable;

mod arg_parser;
mod execution;
mod export_res;
mod interactive_ui;
mod yaml_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = arg_parser::Args::parse();
    let program_mode = args.mode.unwrap_or(ProgramMode::I);
    if program_mode == ProgramMode::I {
        interactive_cli()?;
    } else {
        run_by_config(args.config_loc)?;
    }
    Ok(())
}

fn run_by_config(yaml_config_path: Option<String>) -> Result<(), Box<dyn Error>> {
    let filepath = yaml_config_path.unwrap_or("config.yaml".to_string());
    let yaml_table = TaskTable::load_task_table(filepath)?;
    let executor: Executor<Config> = Executor::new(
        yaml_table.new_element(),
        &Path::new(yaml_table.model_path()),
        yaml_table.target_bondlength(),
    );
    executor.run(&yaml_table)
}

fn interactive_cli() -> Result<(), Box<dyn Error>> {
    // CLI interpretation
    let run_options = RunOptions::new().unwrap();
    let cwd = env!("CARGO_MANIFEST_DIR");
    let radius = run_options.target_bondlength();
    let new_element = run_options.new_element();
    // Task start
    // Read from cell
    // let cell_filepath = Path::new(run_options.filepath());
    // let cell_model = read_from_cell(cell_filepath)?;
    // let cell_seedname = cell_filepath.file_stem().unwrap();
    // let final_report = search_workflow(new_element, radius, cell_filepath)?;
    // let export_options = ExportOptions::new(
    //     run_options.new_element(),
    //     run_options.target_bondlength(),
    //     cell_seedname.to_str().unwrap(),
    // )
    // .unwrap();
    // let export_loc = export_options.export_dir();
    // let potential_loc_path = export_options.potential_dir();
    // let edft = export_options.edft();
    // match export_options.run_mode() {
    //     RunMode::Debug => {
    //         println!("{}", cwd);
    //         println!("{}", potential_loc_path);
    //     }
    //     RunMode::Fast => {
    //         let export_manager = ExportManager::new(
    //             new_element.symbol(),
    //             &export_loc,
    //             &potential_loc_path,
    //             cell_seedname.to_str().unwrap(),
    //             edft,
    //         );
    //         export_manager.overall_in_one(&final_report, &cell_model)?;
    //         export_manager.export_sphere_model(&final_report, &cell_model)?;
    //         export_manager.export_circles_model(&final_report, &cell_model)?;
    //         export_manager.export_points_model(&final_report, &cell_model)?;
    //     }
    //     RunMode::Dryrun => {
    //         println!("Scanning model: {}", cell_filepath.to_str().unwrap());
    //         println!(
    //             "New element: {}, Target bondlength: {} Å",
    //             new_element.symbol(),
    //             radius
    //         );
    //         // let available_elements = mount_checker.available_elements(cell_model.atoms());
    //         // println!("Available bonding elements:");
    //         available_elements.iter().for_each(|elm| println!("{elm}"));
    //         println!("---------------------------------");
    //         println!("Number of CN = 1: {}", final_report.sphere_sites().len());
    //         println!("Located as Spheres");
    //         // final_report.sphere_sites().iter().for_each(|p| {
    //         //     println!(
    //         //         "Sphere center: {}, around atom No.: {}",
    //         //         p.sphere().center,
    //         //         p.locating_atom_id()
    //         //     )
    //         // });
    //         println!("CN = 2: {}", final_report.circles().len());
    //         println!("As circles between two atoms");
    //         // final_report.circles().iter().for_each(|p| {
    //         //     println!(
    //         //         "Located at {}, between atom {} and {}",
    //         //         p.circle().center,
    //         //         p.connecting_atoms()[0],
    //         //         p.connecting_atoms()[1]
    //         //     )
    //         // });
    //         println!("Multi points: {}", final_report.multi_cn_points().len());
    //         final_report.multi_cn_points().iter().for_each(|p| {
    //             println!(
    //                 "Coord: {:#}, Connecting: {:?}",
    //                 p.coord(),
    //                 p.connecting_atom_ids()
    //             )
    //         });
    //     }
    //     RunMode::Post => {
    //         post_copy_potentials(&export_loc, &potential_loc_path)?;
    //     }
    //     RunMode::Clean => {
    //         let target_dir = format!("{}_{}_to_SAC_GDY", new_element.symbol(), radius);
    //         if Path::new(&target_dir).exists() {
    //             Command::new("rm")
    //                 .args(["-r", &target_dir])
    //                 .output()
    //                 .expect(&format!("Error while deleting '{}'", &target_dir));
    //         }
    //     }
    //     RunMode::Full => {
    //         let export_manager = ExportManager::new(
    //             new_element.symbol(),
    //             &export_loc,
    //             &potential_loc_path,
    //             cell_seedname.to_str().unwrap(),
    //             edft,
    //         );
    //         export_manager.export_sphere_model(&final_report, &cell_model)?;
    //         export_manager.export_circles_model(&final_report, &cell_model)?;
    //         export_manager.export_points_model(&final_report, &cell_model)?;
    //         post_copy_potentials(&export_loc, &potential_loc_path)?;
    //     }
    // };
    Ok(())
}
