use rand::Rng; // Import the rand crate for random number generation

fn estimate_pi(num_points: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let mut inside_circle = 0;

    for _ in 0..num_points {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);
        let distance_squared = x * x + y * y;

        if distance_squared <= 1.0 {
            inside_circle += 1;
        }
    }

    // π is estimated as 4 times the ratio of points inside the circle to total points
    let pi_estimate = 4.0 * inside_circle as f64 / num_points as f64;
    pi_estimate
}

fn main() {
    let num_points = 1_000_000;
    let pi_estimate = estimate_pi(num_points);
    println!("Estimated value of π using Monte Carlo: {}", pi_estimate);
}
