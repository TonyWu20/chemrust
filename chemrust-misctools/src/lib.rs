use glob::glob;
use std::{
    error::Error,
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
NP_PER_NODE=12
OMP_NUM_THREADS=1
RUN="RAW"

"#,
        num_nodes,
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

/// Scan the generated `msi` files, create a perl script to be run in `Materials Studio`
/// to save as `xsd` format.
pub fn to_xsd_scripts(target_root_dir: &str) -> Result<(), Box<dyn Error>> {
    let msi_pattern = format!("{target_root_dir}/**/*.msi");
    let item_collection = glob(&msi_pattern)
        .expect("Failed to read glob pattern")
        .into_iter()
        .map(|entry| -> Option<String> {
            match entry {
                Ok(path) => {
                    let stem = path.file_stem().unwrap();
                    let parent = path.parent().unwrap();
                    Some(format!(
                        r#""{}/{}""#,
                        parent.to_str().unwrap(),
                        stem.to_str().unwrap()
                    ))
                }
                Err(e) => {
                    println!("glob entry match error: {:?}", e);
                    None
                }
            }
        })
        .collect::<Vec<Option<String>>>()
        .iter()
        .map(|entry| -> String { entry.as_ref().unwrap().to_string() })
        .collect::<Vec<String>>();
    let all_files_text = item_collection.join(", ");
    let headlines = r#"#!perl
use strict;
use Getopt::Long;
use MaterialsScript qw(:all);
"#;
    let array_text = format!("my @params = (\n{});\n", all_files_text);
    let actions = r#"foreach my $item (@params) {
    my $doc = $Documents{"${item}.msi"};
    $doc->CalculateBonds;
    $doc->Export("${item}.xsd");
    $doc->Save;
    $doc->Close;
}"#;
    let contents = format!("{headlines}{array_text}{actions}");
    fs::write(Path::new("msi_to_xsd.pl"), contents)?;
    Ok(())
}
