use rand::Rng;

const PI: f64 = 3.1415926;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_real<R: Rng>(rng: &mut R) -> (f64, &mut R) {
    (rng.r#gen(), rng)
}

pub fn random_real_interval<R: Rng>(rng: &mut R, min: f64, max: f64) -> (f64, &mut R) {
    (min + rng.r#gen::<f64>() * (max - min), rng)
}