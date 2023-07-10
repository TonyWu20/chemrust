#![allow(dead_code)]
use std::{fmt::Display, str::FromStr};

use castep_periodic_table::element::Element;
use inquire::{Confirm, CustomType, InquireError, Select, Text};
enum KPointQuality {
    Coarse,
    Medium,
    Fine,
}

impl Display for KPointQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KPointQuality::Coarse => write!(f, "Coarse"),
            KPointQuality::Medium => write!(f, "Medium"),
            KPointQuality::Fine => write!(f, "Fine"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseKPointQualityError;

impl FromStr for KPointQuality {
    type Err = ParseKPointQualityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Coarse" => Ok(KPointQuality::Coarse),
            "Medium" => Ok(KPointQuality::Medium),
            "Fine" => Ok(KPointQuality::Fine),
            _ => Err(ParseKPointQualityError),
        }
    }
}
pub struct RunOptions {
    filepath: String,
    new_element: Element,
    target_bondlength: f64,
    edft: bool,
    kpoint_quality: KPointQuality,
}

impl RunOptions {
    fn new() -> Result<RunOptions, InquireError> {
        let filename = Text::new("Filepath of the model cell file:").prompt()?;
        let new_element =
            CustomType::<Element>::new("Element symbol of the new atom: ").prompt()?;
        let target_bondlength = CustomType::<f64>::new("What is the target bondlength (Å)?").with_error_message("Please type a valid number").with_help_message("Type the desired bondlength between the new element atom and the existing atoms in model").prompt()?;
        let edft = Confirm::new(
            "Use edft method for the `metals_method` option in CASTEP?(y/n or yes/no)",
        )
        .prompt()?;
        let kpoint_options: Vec<String> = vec!["Coarse".into(), "Medium".into(), "Fine".into()];
        let kpoint_quality =
            Select::new("Quality for k-point sampling?", kpoint_options).prompt()?;
        let kpoint_quality =
            KPointQuality::from_str(&kpoint_quality).unwrap_or(KPointQuality::Coarse);
        Ok(RunOptions {
            filepath: filename,
            new_element: new_element.clone(),
            target_bondlength,
            edft,
            kpoint_quality,
        })
    }

    pub fn filepath(&self) -> &str {
        self.filepath.as_ref()
    }

    pub fn edft(&self) -> bool {
        self.edft
    }

    fn kpoint_quality(&self) -> &KPointQuality {
        &self.kpoint_quality
    }

    pub fn new_element(&self) -> &Element {
        &self.new_element
    }

    pub fn target_bondlength(&self) -> f64 {
        self.target_bondlength
    }
}

#[test]
fn test_prompts() {
    let options = RunOptions::new().unwrap();
    println!(
        "Filename: {}, new_element: {}, bondlength: {}, edft: {}, kpoint_quality: {}",
        options.filepath(),
        options.new_element().symbol(),
        options.target_bondlength(),
        options.edft(),
        options.kpoint_quality()
    )
}
