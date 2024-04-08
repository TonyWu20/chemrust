use std::fs;
use std::{error::Error, path::Path};

use clap::Parser;

use interactive_ui::RunOptions;

use crate::arg_parser::ProgramMode;
use crate::execution::Executor;
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
    let executor: Executor = Executor::new(
        yaml_table.new_element(),
        &Path::new(yaml_table.model_path()),
        yaml_table.target_bondlength(),
    );
    executor.run(&yaml_table)
}

fn interactive_cli() -> Result<(), Box<dyn Error>> {
    // CLI interpretation
    let run_options = RunOptions::new().unwrap();
    let yaml_table = run_options.export_config()?;
    let executor: Executor = Executor::new(
        yaml_table.new_element(),
        &Path::new(yaml_table.model_path()),
        yaml_table.target_bondlength(),
    );
    executor.run(&yaml_table)?;
    let export_table_filename = format!(
        "{}/{}.yaml",
        yaml_table.export_dir(),
        yaml_table.export_dir()
    );
    fs::write(export_table_filename, serde_yaml::to_string(&yaml_table)?)?;
    Ok(())
}
