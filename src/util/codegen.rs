use rand::Rng;

pub fn generate() -> String {
    let mut rng = rand::rng();
    format!("{:04}-{:04}", rng.random_range(0..=9999), rng.random_range(0..=9999))
}
