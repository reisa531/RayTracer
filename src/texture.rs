pub use crate::vec3::Point3;
pub use crate::color::Color;

pub use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn color_at(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color
}

pub struct CheckerTexture {
    inv_scale: f64,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self {albedo}
    }
}

impl Texture for SolidColor {
    fn color_at(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

impl CheckerTexture {
    pub fn new(scale: f64, odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd: odd.clone(),
            even: even.clone()
        }
    }

    pub fn from_colors(scale: f64, odd_color: Color, even_color: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd: Arc::new(SolidColor::new(odd_color)),
            even: Arc::new(SolidColor::new(even_color))
        }
    }
}

impl Texture for CheckerTexture {
    fn color_at(&self, u: f64, v: f64, p: &Point3) -> Color {
        let ix = (self.inv_scale * p.x()).floor() as i32;
        let iy = (self.inv_scale * p.y()).floor() as i32;
        let iz = (self.inv_scale * p.z()).floor() as i32;

        if (ix + iy + iz) % 2 == 0 {
            return self.even.color_at(u, v, p);
        }
        else {
            return self.odd.color_at(u, v, p);
        }
    }
}