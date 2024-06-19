use std::{fs::read_to_string, path::Path};

use castep_cell_io::{
    CellDocument, CellParseError, CellParser, IonicPosition, IonicPositionBlock, LatticeCart,
    LatticeParam, LatticeParamBlock, LengthUnit, PositionsKeywords,
};
use nalgebra::{Matrix3, Point3};

use crate::data::{
    atom::{Atoms, CoreAtomData},
    geom::coordinates::CoordData,
    lattice::{
        cell_param::{CellConstants, LatticeVectors, UnitCellParameters},
        CrystalModel, LatticeCell,
    },
};

impl LatticeCell {
    pub fn from_cell_file<P: AsRef<Path>>(cell_file_path: P) -> Result<Self, CellParseError> {
        let file_content =
            read_to_string(cell_file_path).expect("Failed to read from given cell file path");
        let cell_doc = CellParser::from(&file_content).parse()?;
        Ok(Self::from(cell_doc))
    }
}

/// Integrations with `castep-cell-io`
impl From<CellDocument> for LatticeCell {
    fn from(value: CellDocument) -> Self {
        let lattice_param = match value.lattice().parameter() {
            castep_cell_io::LatticeParam::LatticeCart(lat_cart) => {
                let matrix = Matrix3::from_column_slice(
                    &[lat_cart.a(), lat_cart.b(), lat_cart.c()].concat(),
                );
                LatticeVectors::new(matrix)
            }
            castep_cell_io::LatticeParam::LatticeABC(lat_abc) => {
                let abc = CellConstants::new(
                    lat_abc.a(),
                    lat_abc.b(),
                    lat_abc.c(),
                    lat_abc.alpha().value(),
                    lat_abc.beta().value(),
                    lat_abc.gamma().value(),
                );
                LatticeVectors::from(abc)
            }
        };
        let indices = value
            .ionic_positions()
            .positions()
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        let symbols = value
            .ionic_positions()
            .positions()
            .iter()
            .map(|pos| pos.symbol())
            .collect();
        let coordinates = value
            .ionic_positions()
            .positions()
            .iter()
            .map(|pos| CoordData::Fractional(Point3::from_slice(&pos.coordinate())))
            .collect();
        let labels = (0..value.ionic_positions().positions().len())
            .map(|_| None)
            .collect();
        let atoms = Atoms::new(indices, symbols, coordinates, labels);
        Self::new(lattice_param, atoms)
    }
}

impl From<LatticeCell> for CellDocument {
    fn from(value: LatticeCell) -> Self {
        let cell_tensor = value.get_cell_parameters().cell_tensor();
        let [a, b, c]: [[f64; 3]; 3] = cell_tensor
            .column_iter()
            .map(|c| c.into())
            .collect::<Vec<[f64; 3]>>()
            .try_into()
            .unwrap();
        let lattice_param = LatticeParam::LatticeCart(LatticeCart::new(a, b, c));
        let lattice_param_block = LatticeParamBlock::new(LengthUnit::default(), lattice_param);
        let atom_data = value.get_atom_data();
        let ionic_positions = atom_data
            .symbols()
            .iter()
            .zip(atom_data.coords())
            .map(|(&elm, coord)| {
                let coordinate: [f64; 3] = if coord.is_fractional() {
                    coord.raw_data().into()
                } else {
                    coord.cart_to_frac(&cell_tensor).raw_data().into()
                };
                IonicPosition::new(elm, coordinate, None)
            })
            .collect::<Vec<IonicPosition>>();
        let ionic_position_block = IonicPositionBlock::new(
            LengthUnit::default(),
            ionic_positions,
            PositionsKeywords::POSITIONS_FRAC,
            true,
        );
        CellDocument::new(lattice_param_block, ionic_position_block)
    }
}
