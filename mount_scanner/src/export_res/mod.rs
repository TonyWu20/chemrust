#![allow(dead_code)]
use std::{
    error::Error,
    fs::{self, read_to_string},
    io,
};

use chemrust_core::data::{Atom, BasicLatticeModel};
use chemrust_formats::{
    castep_param::{BandStructureParam, GeomOptParam},
    seed_writer::{MetalMethodsControl, SeedWriter},
    Cell, FileExport, Msi, StructureFile,
};
use chemrust_parser::CellParser;
use chemrust_scanner::PointStage;
use glob::glob;
use rayon::prelude::*;

const CWD: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Debug, Clone)]
pub struct ExportManager {
    new_element_symbol: String,
    export_loc_str: String,
    potential_loc_str: String,
    edft: bool,
}

impl ExportManager {
    pub fn new(
        new_element_symbol: &str,
        export_loc_str: &str,
        potential_loc_str: &str,
        edft: bool,
    ) -> Self {
        Self {
            new_element_symbol: new_element_symbol.into(),
            export_loc_str: export_loc_str.into(),
            potential_loc_str: potential_loc_str.into(),
            edft,
        }
    }
    fn generate_seed_file(
        &self,
        cell_file: StructureFile<Cell>,
        cell_name: &str,
    ) -> Result<(), io::Error> {
        let geom_seed_writer = SeedWriter::<GeomOptParam>::build(cell_file)
            .with_seed_name(cell_name)
            .with_export_loc(&self.export_loc_str)
            .with_potential_loc(&self.potential_loc_str)
            .build_edft(self.edft);
        geom_seed_writer.write_seed_files()?;
        copy_smcastep_extension(&geom_seed_writer)?;
        let bs_writer: SeedWriter<BandStructureParam> = geom_seed_writer.into();
        bs_writer.write_seed_files()?;
        Ok(())
    }
    pub fn overall_in_one(
        &self,
        final_report: &PointStage,
        original_lattice_model: &BasicLatticeModel,
        lattice_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let spheres = final_report.visualize_specific_sites(final_report.sphere_sites());
        let circles = final_report.visualize_specific_sites(final_report.circles());
        let points = final_report.visualize_specific_sites(
            &[final_report.cut_points(), final_report.multi_cn_points()].concat(),
        );
        if spheres.len() > 0 {
            self.export_per_sites(
                &spheres,
                original_lattice_model,
                lattice_name,
                "single_spheres",
            )?;
            println!("Export all single sites")
        }
        if circles.len() > 0 {
            self.export_per_sites(
                &circles,
                original_lattice_model,
                lattice_name,
                "double_circles",
            )?;
            println!("Export all double circles")
        }
        if points.len() > 0 {
            self.export_per_sites(
                &points,
                original_lattice_model,
                lattice_name,
                "multi_points",
            )?;
            println!("Export all multi points")
        }
        Ok(())
    }
    fn visualize_per_sites(
        &self,
        new_atoms: &[Atom],
        original_lattice_model: &BasicLatticeModel,
    ) -> StructureFile<Cell> {
        let new_atoms = [original_lattice_model.atoms(), new_atoms].concat();
        let new_lattice_vectors = original_lattice_model.lattice_vectors().unwrap().clone();
        let new_lattice = BasicLatticeModel::new(&Some(new_lattice_vectors), &new_atoms);
        StructureFile::<Cell>::new(new_lattice)
    }
    fn export_per_sites(
        &self,
        new_atoms: &[Atom],
        original_lattice_model: &BasicLatticeModel,
        lattice_name: &str,
        site_name: &str,
    ) -> Result<(), io::Error> {
        let new_cell = self.visualize_per_sites(new_atoms, original_lattice_model);
        let new_text = [new_cell.write_lattice_vectors(), new_cell.write_atoms()].join("\n");
        let new_visualize_msi: StructureFile<Msi> = new_cell.into();
        let export_name = format!("{lattice_name}_all_{site_name}_demo");
        new_visualize_msi.write_to(&format!("{}.msi", &export_name))?;
        fs::write(&format!("{export_name}.cell"), new_text)
    }
    pub fn export_sphere_model(
        &self,
        final_report: &PointStage,
        original_lattice_model: &BasicLatticeModel,
        lattice_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(spheres_res) =
            final_report.generate_sphere_models(original_lattice_model, &self.new_element_symbol)
        {
            spheres_res.into_iter().try_for_each(|(name, model)| {
                let cell_output = StructureFile::<Cell>::new(model);
                let export_name = format!("{}_{}", lattice_name, name);
                self.generate_seed_file(cell_output, &export_name)
            })?
        }
        Ok(())
    }
    pub fn export_circles_model(
        &self,
        final_report: &PointStage,
        original_lattice_model: &BasicLatticeModel,
        lattice_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(circle_res) =
            final_report.generate_circle_models(original_lattice_model, &self.new_element_symbol)
        {
            circle_res.into_iter().try_for_each(|(name, model)| {
                let cell_output = StructureFile::<Cell>::new(model);
                let export_name = format!("{}_{}", lattice_name, name);
                self.generate_seed_file(cell_output, &export_name)
            })?
        }
        Ok(())
    }
    pub fn export_points_model(
        &self,
        final_report: &PointStage,
        original_lattice_model: &BasicLatticeModel,
        lattice_name: &str,
    ) -> Result<(), io::Error> {
        if let Some(cut_point_res) =
            final_report.generate_cut_point_models(original_lattice_model, &self.new_element_symbol)
        {
            cut_point_res.into_iter().try_for_each(|(name, model)| {
                let cell_output = StructureFile::<Cell>::new(model);
                let export_name = format!("{}_{}", lattice_name, name);
                self.generate_seed_file(cell_output, &export_name)
            })?
        }
        if let Some(multi_point_res) = final_report
            .generate_multi_point_models(original_lattice_model, &self.new_element_symbol)
        {
            multi_point_res.into_iter().try_for_each(|(name, model)| {
                let cell_output = StructureFile::<Cell>::new(model);
                let export_name = format!("{}_{}", lattice_name, name);
                self.generate_seed_file(cell_output, &export_name)
            })?
        }
        Ok(())
    }
}

/// Copy the extension and rename to the model name.
fn copy_smcastep_extension(writer: &SeedWriter<GeomOptParam>) -> Result<(), io::Error> {
    let dest_dir = writer.create_export_dir()?;
    let with_seed_name = format!("SMCastep_Extension_{}.xms", writer.seed_name());
    let dest_path = dest_dir.join(&with_seed_name);
    if !dest_path.exists() {
        fs::copy(
            &format!("{}/../resources/SMCastep_Extension.xms", CWD),
            dest_path,
        )?;
    }
    Ok(())
}
pub fn post_copy_potentials(
    target_directory: &str,
    potential_loc_str: &str,
) -> Result<(), io::Error> {
    let msi_pattern = format!("{target_directory}/**/*.msi");
    glob(&msi_pattern)
        .unwrap()
        .into_iter()
        .par_bridge()
        .try_for_each(|entry| -> Result<(), io::Error> {
            let cell_entry = entry.as_ref().unwrap().with_extension("cell");
            let content = read_to_string(cell_entry).unwrap();
            let cell_model = CellParser::new(&content)
                .to_lattice_cart()
                .to_positions()
                .build_lattice();
            let cell_output = StructureFile::<Cell>::new(cell_model);
            let filepath = entry.as_ref().unwrap().clone();
            let dir_path = filepath
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .clone();
            let cell_name = filepath.file_stem().unwrap().to_str().unwrap().to_owned();
            let writer: SeedWriter<GeomOptParam> = SeedWriter::build(cell_output)
                .with_seed_name(&cell_name)
                .with_export_loc(dir_path)
                .with_potential_loc(potential_loc_str)
                .build();
            writer.copy_potentials()?;
            Ok(())
        })?;
    Ok(())
}
