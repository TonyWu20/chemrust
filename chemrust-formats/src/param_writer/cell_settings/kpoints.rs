use std::fmt::Display;

use crate::Cell;

#[derive(Debug, Clone)]
pub struct KPointsList {
    kpts: Vec<[f64; 4]>,
}

impl KPointsList {
    pub fn write_kpoints_list(&self) -> String {
        Cell::write_block(("KPOINTS_LIST".into(), format!("{self}")))
    }
    pub fn write_bs_kpoints_list(&self) -> String {
        Cell::write_block(("BS_KPOINTS_LIST".into(), format!("{self}")))
    }
}

impl Default for KPointsList {
    fn default() -> Self {
        Self {
            kpts: vec![[0.0, 0.0, 0.0, 1.0]],
        }
    }
}

impl Display for KPointsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .kpts
            .iter()
            .map(|kpt| {
                kpt.iter()
                    .map(|num| format!("{:20.16}", num))
                    .collect::<Vec<String>>()
                    .concat()
            })
            .collect::<Vec<String>>()
            .join("\n");
        writeln!(f, "{text}")
    }
}

#[cfg(test)]
mod test {
    use super::KPointsList;

    #[test]
    fn test_kpts() {
        let kpts_list = KPointsList::default();
        println!("{}", kpts_list.write_kpoints_list());
    }
}
