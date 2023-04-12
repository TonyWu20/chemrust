fn fractional_numbers(number_of_points: u32) -> Vec<u32> {
    let r_ranges = (0..number_of_points).into_iter();
    r_ranges
        .map(|r| (2 * r - number_of_points - 1) / (2 * number_of_points))
        .collect()
}
