use crate::types::{point, Pixel, Point, Rectangle};

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub bounds: Rectangle,
    pub scaled_bounds: Rectangle,
}

impl Image {
    pub fn check_pixel(&self, point: Point) -> bool {
        if !self.bounds.contains(point) {
            return false;
        }

        let x = (point.x.floor() - self.bounds.x) * 0.05;
        let y = (point.y.floor() - self.bounds.y) * 0.05;
        let idx = (((y.floor() * self.scaled_bounds.width) + x.floor())) as usize;

        if idx > self.pixels.len() {
            return false;
        }

        self.pixels[idx].a >= 192
    }
}
