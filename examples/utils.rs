use rand::Rng;

pub fn generate_random_coordinates(n_coordinates: usize, grid_size: usize) -> Vec<(f64, f64)> {
    let mut rng = rand::rng();
    (0..n_coordinates)
        .map(|_| {
            (
                rng.random_range(0.0..grid_size as f64),
                rng.random_range(0.0..grid_size as f64),
            )
        })
        .collect()
}

// Function to calculate the Euclidean distance between two points
pub fn euclidean_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;

    (dx * dx + dy * dy).sqrt()
}
