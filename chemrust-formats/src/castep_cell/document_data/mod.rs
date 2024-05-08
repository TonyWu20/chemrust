pub enum ValueType<'a> {
    Block,
    Integer(i32),
    Real(f64),
    String(&'a str),
    Logical(bool),
}

pub enum BlockData {
    LatticeABC([f64; 6]),
    LatticeCart([f64; 9]),
    PositionsFrac,
    PositionsAbs,
}

mod positions_data;
mod units;
