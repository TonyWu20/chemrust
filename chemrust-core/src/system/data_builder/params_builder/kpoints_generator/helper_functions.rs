pub fn grid_size_determine(length: f64, spacing: f64) -> u32 {
    let div = length / spacing;
    let rounded = div.round();
    if rounded >= 1.0 {
        rounded as u32
    } else {
        1
    }
}

pub fn mp_grid_generate(norms: &[f64; 3], spacing: f64) -> [u32; 3] {
    let grid: Vec<u32> = norms
        .iter()
        .map(|&i| grid_size_determine(i, spacing))
        .collect();
    grid.try_into().unwrap()
}
