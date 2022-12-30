use chemrust_core::data::{format::msi::Msi, AtomId, AtomicNumber, CartesianCoord, ElementSymbol};
use nalgebra::Point3;

pub(crate) trait FromRawAttrs {
    type Output;
    fn convert(self) -> Self::Output;
}

impl FromRawAttrs for Vec<u8> {
    type Output = Vec<AtomicNumber<Msi>>;

    fn convert(self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(i, num)| AtomicNumber::new(num, i))
            .collect()
    }
}

impl FromRawAttrs for Vec<Point3<f64>> {
    type Output = Vec<CartesianCoord<Msi>>;
    fn convert(self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(i, xyz)| CartesianCoord::new(xyz, i))
            .collect()
    }
}

impl<'s> FromRawAttrs for Vec<&'s str> {
    type Output = Vec<ElementSymbol<Msi>>;

    fn convert(self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(i, symbol)| ElementSymbol::new(symbol.into(), i))
            .collect()
    }
}

impl FromRawAttrs for Vec<u32> {
    type Output = Vec<AtomId<Msi>>;
    fn convert(self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(i, id)| AtomId::new(id, i))
            .collect()
    }
}
