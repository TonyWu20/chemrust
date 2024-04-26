use crate::data::atom::CoreAtomData;

use crate::data::lattice::cell_param::UnitCellParameters;

/// The struct to represent a crystal model structure should implement this trait.
/// # Examples
/// ```
/// use std::f64::consts::FRAC_PI_4;
/// use castep_periodic_table::element::ElementSymbol;
/// use nalgebra::Point3;

/// use chemrust_core::data::{
///     atom::CoreAtomData,
///     geom::coordinates::CoordData,
///     lattice::{
///         cell_param::{CellConstants, UnitCellParameters},
///         CrystalModel,
///     },
/// };
/// let lattice_param = CellConstants::new(9.99, 9.99, 9.99, FRAC_PI_4, FRAC_PI_4, FRAC_PI_4);
/// struct Atoms {
///     indices: Vec<usize>,
///     symbols: Vec<ElementSymbol>,
///     frac: Vec<CoordData>,
///     label: Vec<Option<String>>,
/// }
/// let atoms = Atoms {
///     indices: vec![0, 1],
///     symbols: vec![ElementSymbol::H, ElementSymbol::Li],
///     frac: vec![
///         CoordData::Fractional(Point3::new(0.0, 0.0, 0.0)),
///         CoordData::Fractional(Point3::new(0.5, 0.5, 0.5)),
///     ],
///     label: vec![None, None],
/// };
/// impl CoreAtomData for Atoms {
///     fn indices(&self) -> &[usize] {
///         &self.indices
///     }
///     fn symbols(&self) -> &[ElementSymbol] {
///         &self.symbols
///     }
///     fn coords(&self) -> &[CoordData] {
///         &self.frac
///     }
///     fn labels(&self) -> &[Option<String>] {
///         &self.label
///     }
/// }
/// struct CellStructure {
///     params: CellConstants,
///     atoms: Atoms,
/// }
///
/// impl CrystalModel for CellStructure {
///     fn get_cell_parameters(&self) -> &impl UnitCellParameters {
///             &self.params
///     }
///
///     fn get_atom_data(&self) -> &impl CoreAtomData {
///             &self.atoms
///     }
/// }
/// let cell_struct = CellStructure {
///     params: lattice_param,
///     atoms,
/// };
/// let _cell_tensor = cell_struct.get_cell_parameters().cell_tensor();
/// let _atoms_coords = cell_struct.get_atom_data().coords();
/// ```
pub trait CrystalModel {
    fn get_cell_parameters(&self) -> &impl UnitCellParameters;
    fn get_atom_data(&self) -> &impl CoreAtomData;
}
