use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

pub fn ideal_bondlength(atomic_num_1: u8, atomic_num_2: u8) -> f64 {
    let cov_rad_1 = ELEMENT_TABLE
        .get_by_atomic_number(atomic_num_1)
        .unwrap()
        .covalent_radius()
        .unwrap_or(0.0);
    let cov_rad_2 = ELEMENT_TABLE
        .get_by_atomic_number(atomic_num_2)
        .unwrap()
        .covalent_radius()
        .unwrap_or(0.0);
    cov_rad_1 + cov_rad_2
}

pub fn is_bonded(distance: f64, ideal_bondlength: f64, lower_fac: f64, upper_fac: f64) -> bool {
    let lower = lower_fac * ideal_bondlength;
    let upper = upper_fac * ideal_bondlength;
    // Suggested by `clippy`
    !(distance < lower || distance > upper)
}
