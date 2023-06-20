use std::fmt::Display;

use nalgebra::Matrix4;

use crate::Cell;

#[derive(Debug, Clone)]
pub struct SymmetryOps {
    operations: Vec<Matrix4<f64>>,
}

impl SymmetryOps {
    pub fn write_in_cell(&self) -> String {
        Cell::write_block(("SYMMETRY_OPS".into(), format!("{}", self)))
    }
}

impl Default for SymmetryOps {
    fn default() -> Self {
        Self {
            operations: vec![Matrix4::identity()],
        }
    }
}

impl Display for SymmetryOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ops_output = self
            .operations
            .iter()
            .map(|mat| {
                let rot_part = mat.fixed_view::<3, 3>(0, 0);
                let translate_part = mat.fixed_view::<3, 1>(0, 3);
                let rot_part_output = rot_part
                    .column_iter() // Should be column-major order in .cell? Not sure.
                    .map(|col| {
                        col.iter()
                            .map(|value| format!("{:23.15}", value))
                            .collect::<Vec<String>>()
                            .concat()
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let translate_output = translate_part
                    .iter()
                    .map(|value| format!("{:23.15}", value))
                    .collect::<Vec<String>>()
                    .concat();
                let output = vec![rot_part_output, "\n".into(), translate_output, "\n".into()];
                output.concat()
            })
            .collect::<Vec<String>>()
            .concat();
        write!(f, "{ops_output}")
    }
}

#[cfg(test)]
mod test {
    use super::SymmetryOps;

    #[test]
    fn test_sym_out() {
        let sym_ops = SymmetryOps::default();
        println!("{}", sym_ops.write_in_cell());
    }
}
