use std::{fmt::Display, fs, io, path::Path};

use self::script::{LsfScript, PbsScript, ServerScript};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServerScriptType {
    Pbs,
    Lsf,
}

pub fn write_server_script<P: AsRef<Path>>(
    cell_path: &P,
    num_nodes: u32,
    server_type: ServerScriptType,
) -> Result<(), io::Error> {
    let target_dir = cell_path
        .as_ref()
        .parent()
        .expect("can't find parent of the path");
    let script = match server_type {
        ServerScriptType::Pbs => format!("{}", pbs_script(cell_path, num_nodes)),
        ServerScriptType::Lsf => format!("{}", lsf_script(cell_path, num_nodes)),
    };
    let script_path = match server_type {
        ServerScriptType::Pbs => target_dir.join("hpc.pbs.sh"),
        ServerScriptType::Lsf => target_dir.join("MS70_YW_CASTEP.lsf"),
    };
    fs::write(script_path, script)
}

fn lsf_script<P: AsRef<Path>>(cell_path: &P, num_nodes: u32) -> impl ServerScript + Display {
    let cell_name = cell_path
        .as_ref()
        .file_stem()
        .expect("Failed to get file stem")
        .to_str()
        .expect("Contains invalid unicode");
    let mut lsf_script = LsfScript::default();
    lsf_script.set_cores_per_node(num_nodes);
    lsf_script.set_job_name(cell_name);
    lsf_script
}

fn pbs_script<P: AsRef<Path>>(cell_path: &P, num_nodes: u32) -> impl ServerScript + Display {
    let cell_name = cell_path
        .as_ref()
        .file_stem()
        .expect("Failed to get file stem")
        .to_str()
        .expect("Contains invalid unicode");
    PbsScript::new(num_nodes, cell_name.to_string())
}

mod script;
