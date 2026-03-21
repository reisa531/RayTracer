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
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = (p.x().floor()) as i32;
        let j = (p.y().floor()) as i32;
        let k = (p.z().floor()) as i32;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[
                       (self.perm_x[((i + di as i32) & 255) as usize] ^
                        self.perm_y[((j + dj as i32) & 255) as usize] ^
                        self.perm_z[((k + dk as i32) & 255) as usize]) as usize
                    ];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum: f64 = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as f64) * u + ((1 - i) as f64) * (1.0 - u))
                           * ((j as f64) * v + ((1 - j) as f64) * (1.0 - v))
                           * ((k as f64) * w + ((1 - k) as f64) * (1.0 - w))
                           * c[i][j][k];
                }
            }
        }
        accum
    }
}