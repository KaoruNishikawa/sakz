#[allow(dead_code, unused_variables)]
pub fn reduce(data: Vec<(f64, f64, f64)>, az: f64, el: f64, rot: f64) -> Vec<(f64, f64)> {
    // TODO: Implement reducer
    let reduced = data
        .iter()
        .map(|(x, y, z)| return (*x * az, *y * el))
        .collect();
    reduced
}
