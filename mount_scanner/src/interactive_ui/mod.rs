#![allow(dead_code)]

pub use self::{
    execute_modes::RunMode, export_options::ExportOptions, filepath_completer::FilePathCompleter,
    kpoint_quality::KPointQuality, run_options::RunOptions,
};

mod execute_modes;
mod export_options;
mod filepath_completer;
mod kpoint_quality;
mod run_options;

#[test]
fn test_prompts() {
    let options = RunOptions::new().unwrap();
    let filestem = std::path::Path::new(options.filepath())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let export_options =
        ExportOptions::new(options.new_element(), options.target_bondlength(), filestem).unwrap();
    println!(
        "Filename: {}, new_element: {}, bondlength: {}, edft: {}, kpoint_quality: {}",
        options.filepath(),
        options.new_element().symbol(),
        options.target_bondlength(),
        export_options.edft(),
        export_options.kpoint_quality()
    );
}
