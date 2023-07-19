

use castep_periodic_table::element::Element;
use inquire::{required, validator::Validation, CustomType, InquireError, Text};

use super::{filepath_completer::FilePathCompleter};

#[derive(Debug)]
pub struct RunOptions {
    filepath: String,
    new_element: Element,
    target_bondlength: f64,
}

impl RunOptions {
    fn ask_filename() -> Result<String, InquireError> {
        let current_dir = std::env::current_dir().unwrap();
        let help_message = format!("Current directory: {}", current_dir.to_string_lossy());
        Text::new("Filepath of the model cell file:")
            .with_autocomplete(FilePathCompleter::default())
            .with_validator(required!("This field is required"))
            .with_validator(|input: &str| {
                if input.split('.').last().unwrap() == "cell" {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(
                        inquire::validator::ErrorMessage::Custom(
                            "Please enter the filepath of a `.cell` file".into(),
                        ),
                    ))
                }
            })
            .with_help_message(&help_message)
            .prompt()
    }
    fn ask_element() -> Result<Element, InquireError> {
        CustomType::<Element>::new("Element symbol of the new atom: ").prompt()
    }
    fn ask_bondlength() -> Result<f64, InquireError> {
        CustomType::<f64>::new("What is the target bondlength (Å)?").with_error_message("Please type a valid number").with_help_message("Type the desired bondlength between the new element atom and the existing atoms in model").prompt()
    }
    pub fn new() -> Result<RunOptions, InquireError> {
        let filename = Self::ask_filename()?;
        let new_element = Self::ask_element()?;
        let target_bondlength = Self::ask_bondlength()?;
        Ok(RunOptions {
            filepath: filename,
            new_element: new_element.clone(),
            target_bondlength,
        })
    }

    pub fn filepath(&self) -> &str {
        self.filepath.as_ref()
    }

    pub fn new_element(&self) -> &Element {
        &self.new_element
    }

    pub fn target_bondlength(&self) -> f64 {
        self.target_bondlength
    }
}
