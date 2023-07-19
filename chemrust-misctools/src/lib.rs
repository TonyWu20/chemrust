use std::{
    fs::{self, read_to_string},
    io,
    path::Path,
};

use chemrust_parser::CellParser;

pub fn write_lsf_script<P: AsRef<Path>>(cell_path: &P, num_nodes: u32) -> Result<(), io::Error> {
    let target_dir = cell_path.as_ref().parent().unwrap();
    let cell_name = cell_path.as_ref().file_stem().unwrap().to_str().unwrap();
    let cmd = format!(
        "/home-yw/Soft/msi/MS70/MaterialsStudio7.0/etc/CASTEP/bin/RunCASTEP.sh -np $NP {cell_name}"
    );
    let prefix = format!(
        r#"APP_NAME=intelY_mid
NP={}
NP_PER_NODE={}
OMP_NUM_THREADS=1
RUN="RAW"

"#,
        num_nodes, num_nodes
    );
    let content = format!("{prefix}{cmd}");
    let lsf_filepath = target_dir.join("MS70_YW_CASTEP.lsf");
    fs::write(lsf_filepath, content)
}
pub fn copy_potentials<P: AsRef<Path>>(cell_path: &P, potential_loc: &P) -> Result<(), io::Error> {
    let file = read_to_string(cell_path).unwrap();
    let potentials = CellParser::new(&file)
        .to_potentials()
        .unwrap()
        .report_potential_files();
    let dest_dir = cell_path.as_ref().parent().unwrap();
    potentials
        .iter()
        .try_for_each(|pot_file| -> Result<(), io::Error> {
            let pot_src_path = potential_loc.as_ref().join(pot_file);
            let pot_dest_path = dest_dir.join(pot_file);
            if !pot_dest_path.exists() {
                fs::copy(pot_src_path, pot_dest_path)?;
                Ok(())
            } else {
                Ok(())
            }
        })
}
