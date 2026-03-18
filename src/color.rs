use crate::vec3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

impl Color {
    pub fn print_color(&self) {
        let ir = (256.0 * Interval::PSEUDO_UNIT.clamp(self.x())) as u8;
        let ig = (256.0 * Interval::PSEUDO_UNIT.clamp(self.y())) as u8;
        let ib = (256.0 * Interval::PSEUDO_UNIT.clamp(self.z())) as u8;

        println!("{} {} {}", ir, ig, ib);
    }
}