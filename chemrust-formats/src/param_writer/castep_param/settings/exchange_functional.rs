use std::fmt::Display;

#[derive(Debug)]
pub enum XCFunctional {
    LDA,
    PW91,
    PBE,
    RPBE,
    WC,
    PBESOL,
    HF,
    HFLDA, // HF-LDA
    SX,    // sX
    SXLDA, // sX-LDA
    PBEO,
    B3LYP,
    HSE03,
    HSE06,
}

impl Display for XCFunctional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XCFunctional::LDA => write!(f, "LDA"),
            XCFunctional::PW91 => write!(f, "PW91"),
            XCFunctional::PBE => write!(f, "PBE"),
            XCFunctional::RPBE => write!(f, "RPBE"),
            XCFunctional::WC => write!(f, "WC"),
            XCFunctional::PBESOL => write!(f, "PBESOL"),
            XCFunctional::HF => write!(f, "HF"),
            XCFunctional::HFLDA => write!(f, "HF-LDA"),
            XCFunctional::SX => write!(f, "sX"),
            XCFunctional::SXLDA => write!(f, "sX-LDA"),
            XCFunctional::PBEO => write!(f, "PBEO"),
            XCFunctional::B3LYP => write!(f, "B3LYP"),
            XCFunctional::HSE03 => write!(f, "HSE03"),
            XCFunctional::HSE06 => write!(f, "HSE06"),
        }
    }
}

impl Default for XCFunctional {
    fn default() -> Self {
        XCFunctional::PBE
    }
}
