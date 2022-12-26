use std::fmt::Display;

use crate::{
    data::{AtomCollection, AtomId, AtomicNumber, CartesianCoord, ElementSymbol},
    impl_display,
};

use super::DataFormat;

#[derive(Debug, Clone, Default)]
pub struct Msi;

impl DataFormat for Msi {}

impl_display!(AtomId<Msi>, "{}");
impl_display!(ElementSymbol<Msi>, "{}");
impl_display!(AtomicNumber<Msi>, "{}");
impl_display!(CartesianCoord<Msi>, "({:.12} {:.12} {:.12})", x, y, z);

impl Display for AtomCollection<Msi> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.atom_ids().len();
        let display_string_collections: Vec<String> = (0..size)
            .into_iter()
            .map(|i| {
                format!(
                    r#"  ({item_id} Atom
    (A C ACL "{elm_id} {elm}")
    (A C Label "{elm}")
    (A D XYZ {xyz})
    (A I Id {atom_id})
  )
"#,
                    item_id = self.atom_ids().get(i).unwrap().content() + 1,
                    elm_id = self.atomic_number().get(i).unwrap(),
                    elm = self.element_symbols().get(i).unwrap(),
                    xyz = self.xyz().get(i).unwrap(),
                    atom_id = self.atom_ids().get(i).unwrap()
                )
            })
            .collect();
        write!(f, "{}", display_string_collections.concat())
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Point3;

    use crate::data::{AtomId, AtomicNumber, CartesianCoord, ElementSymbol};

    use super::Msi;

    #[test]
    fn test_msi_display() {
        let atom_id: AtomId<Msi> = AtomId::new(1_u32, 0);
        let element_symbol: ElementSymbol<Msi> = ElementSymbol::new("H".into(), 0);
        let atomic_number: AtomicNumber<Msi> = AtomicNumber::new(0, 0);
        let cart_coord: CartesianCoord<Msi> =
            CartesianCoord::new(Point3::new(0_f64, 0_f64, 0_f64), 0);
        println!("atom_id: {}", atom_id);
        println!("element_symbol: {}", element_symbol);
        println!("atomic_number: {}", atomic_number);
        println!("cart_coord: {}", cart_coord);
    }
}
