pub use rand::Rng;

pub use crate::vec3::Point3;

pub struct Perlin {
    randfloat: [f64; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256]
}

impl Perlin {
    fn permute(p: &mut [i32], n: usize) {
        let mut rng = rand::thread_rng();
        for i in (0..n).rev() {
            let target: usize = rng.gen_range(0..=i);
            let temp = p[i];
            p[i] = p[target];
            p[target] = temp;
        }
    }

    fn perlin_generate_perm(p: &mut [i32]) {
        for i in 0..256 {
            p[i as usize] = i;
        }

        Self::permute(p, 256);
    }

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut randfloat = [0.0; 256];
        for i in 0..256 {
            randfloat[i] = rng.r#gen();
        }

        let mut perm_x = [0; 256];
        let mut perm_y = [0; 256];
        let mut perm_z = [0; 256];

        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        self.randfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}