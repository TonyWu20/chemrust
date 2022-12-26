use crate::data::format::DataFormat;

use super::markers::*;
use std::{fmt::Debug, marker::PhantomData};
#[derive(Debug, Clone)]
pub struct ParamAttr<T, M: ParamMarker, F: DataFormat>(T, PhantomData<M>, PhantomData<F>)
where
    M: ParamMarker,
    F: DataFormat;

macro_rules! def_param_type {
    ($(($type_name: ident,$type: ty,  $marker: ty)), *) => {
       $(pub type $type_name<T> = ParamAttr<$type, $marker,T>;)*
    };
}

def_param_type!(
    (KPoint, [f64; 4], KPointMark),
    (KPointGrid, [u8; 3], KPointGridMark),
    (KPointMPSpacing, f64, KPointSpacingMark),
    (KPointOffset, [f64; 3], KPointOffsetMark),
    (EField, [f64; 3], EFieldMark),
    (EPressure, [f64; 6], EPressureMark),
    (CryDisplay, (u32, u32), CryDisplayMark),
    (PeriodicType, u8, PeriodicTypeMark),
    (SpaceGroup, String, SpaceGroupMark),
    (CryTolerance, f64, CryTolMark)
);

impl<T, M, F> ParamAttr<T, M, F>
where
    M: ParamMarker,
    F: DataFormat,
{
    pub fn content(&self) -> &T {
        &self.0
    }
    pub fn new(input: T) -> Self {
        Self(input, PhantomData, PhantomData)
    }
}

macro_rules! param_defaults {
    ($(($x: ty, $l:expr)), *) => {
        $(
            impl<T:DataFormat> Default for $x {
                fn default() -> Self {
                    Self($l, PhantomData, PhantomData)
                }
            }

            )*
    };
}

param_defaults!(
    (KPoint<T>, [0.0, 0.0, 0.0, 1.0]),
    (KPointGrid<T>, [1,1,1]),
    (KPointMPSpacing<T>, 0.07),
    (KPointOffset<T>, [0.0,0.0,0.0]),
    (EField<T>, [0.0,0.0,0.0]),
    (EPressure<T>, [();6].map(|_| 0.0)),
    (CryTolerance<T>, 0.05),
    (SpaceGroup<T>, "P1 1".into()),
    (PeriodicType<T>, 100),
    (CryDisplay<T>, (192,256))
);
