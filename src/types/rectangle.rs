use super::Point;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x <= self.x + self.width && point.y >= self.y && point.y <= self.y + self.height
    }
}
