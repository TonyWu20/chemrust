use crate::data::atom::CoreAtomData;

use crate::data::lattice::cell_param::UnitCellParameters;

/// The struct to represent a crystal model structure should implement this trait.
pub trait CrystalModel: UnitCellParameters + CoreAtomData {}

#[cfg(test)]
mod test {
    use castep_periodic_table::element::ElementSymbol;

    use crate::data::{atom::CoreAtomData, geom::coordinates::CoordData, lattice::LatticeVectors};

    use super::CrystalModel;

    /// Basic example struct
    #[derive(Debug, Clone)]
    struct Atoms {
        indices: Vec<usize>,
        symbols: Vec<ElementSymbol>,
        coordinates: Vec<CoordData>,
        labels: Vec<Option<String>>,
    }

    impl Atoms {
        pub fn new(
            indices: Vec<usize>,
            symbols: Vec<ElementSymbol>,
            coordinates: Vec<CoordData>,
            labels: Vec<Option<String>>,
        ) -> Self {
            Self {
                indices,
                symbols,
                coordinates,
                labels,
            }
        }
    }

    impl CoreAtomData for Atoms {
        fn indices_repr(&self) -> Vec<usize> {
            self.indices.clone()
        }

        fn symbols_repr(&self) -> Vec<ElementSymbol> {
            self.symbols.clone()
        }

        fn coords_repr(&self) -> Vec<CoordData> {
            self.coordinates.clone()
        }

        fn labels_repr(&self) -> Vec<Option<String>> {
            self.labels.clone()
        }
    }

    struct ChemModel {
        lattice: LatticeVectors,
        atoms: Atoms,
    }

    trait Loader {
        fn load_model(&self) -> Box<dyn CrystalModel>;
    }

    impl CrystalModel for ChemModel {}

    impl crate::data::lattice::UnitCellParameters for ChemModel {
        fn cell_volume(&self) -> f64 {
            <LatticeVectors as crate::data::lattice::UnitCellParameters>::cell_volume(&self.lattice)
        }

        fn lattice_bases(&self) -> nalgebra::Matrix3<f64> {
            <LatticeVectors as crate::data::lattice::UnitCellParameters>::lattice_bases(
                &self.lattice,
            )
        }
    }

    impl CoreAtomData for ChemModel {
        fn indices_repr(&self) -> Vec<usize> {
            <Atoms as CoreAtomData>::indices_repr(&self.atoms)
        }

        fn symbols_repr(&self) -> Vec<ElementSymbol> {
            <Atoms as CoreAtomData>::symbols_repr(&self.atoms)
        }

        fn coords_repr(&self) -> Vec<CoordData> {
            <Atoms as CoreAtomData>::coords_repr(&self.atoms)
        }

        fn labels_repr(&self) -> Vec<Option<String>> {
            <Atoms as CoreAtomData>::labels_repr(&self.atoms)
        }
    }
}
