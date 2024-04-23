use std::{error::Error, fs, path::Path};

use castep_periodic_table::element::Element;
use chemrust_core::data::{custom_data_type::FractionalCoordRange, BasicLatticeModel};
use chemrust_parser::CellParser;
use chemrust_scanner::{FinalReport, MountingChecker};

use crate::{export_res::ExportManager, yaml_parser::TaskTable};

mod run_modes;

#[derive(Debug)]
pub struct Executor<'a> {
    new_element: &'a Element,
    cell_filepath: &'a Path,
    cell_model: BasicLatticeModel,
    radius: f64,
}

impl<'a> Executor<'a> {
    pub fn new(new_element: &'a Element, cell_filepath: &'a Path, radius: f64) -> Self {
        let cell_text = fs::read_to_string(cell_filepath).unwrap();
        let cell_model = CellParser::new(&cell_text)
            .to_lattice_cart()
            .to_positions()
            .build_lattice();
        Self {
            new_element,
            cell_filepath,
            cell_model,
            radius,
        }
    }

    fn search(
        &self,
        x_range: FractionalCoordRange,
        y_range: FractionalCoordRange,
        z_range: FractionalCoordRange,
    ) -> Result<FinalReport, Box<dyn Error>> {
        let mount_checker = MountingChecker::new_builder()
            .with_element(self.new_element)
            .with_bondlength(self.radius)
            .build();
        let filtered_atoms = self.cell_model.xyz_range_filter(x_range, y_range, z_range);
        if !filtered_atoms.is_empty() {
            Ok(mount_checker.mount_search(&filtered_atoms))
        } else {
            panic!("No atoms found in this range")
        }
    }
    fn export_manager(&self, export_loc: &str, potential_loc: &str, edft: bool) -> ExportManager {
        let lattice_name = self.cell_filepath.file_stem().unwrap().to_str().unwrap();
        let p = Path::new(export_loc);
        if !p.exists() {
            println!("{:?}", p);
            fs::create_dir(p).unwrap()
        } else {
            println!("Has {:?}", p);
        }
        ExportManager::new(
            self.new_element.symbol(),
            export_loc,
            potential_loc,
            lattice_name,
            edft,
        )
    }
    fn export(
        &self,
        export_loc: &str,
        potential_loc: &str,
        edft: bool,
        final_stage: &FinalReport,
    ) -> Result<(), Box<dyn Error>> {
        let manager = self.export_manager(export_loc, potential_loc, edft);
        manager.export_points_model(final_stage, &self.cell_model)?;
        manager.export_circles_model(final_stage, &self.cell_model)?;
        manager.export_sphere_model(final_stage, &self.cell_model)?;
        manager.overall_in_one(final_stage, &self.cell_model)?;
        Ok(())
    }
}

impl<'a> Executor<'a> {
    pub fn run(&self, config_table: &TaskTable) -> Result<(), Box<dyn Error>> {
        let final_stage = self.search(
            config_table.x_range(),
            config_table.y_range(),
            config_table.z_range(),
        )?;
        let cwd = env!("CARGO_MANIFEST_DIR");
        self.export(
            config_table.export_dir(),
            config_table
                .potential_dir()
                .unwrap_or(&format!("{}/../Potentials", cwd)),
            config_table.edft(),
            &final_stage,
        )
    }
}
