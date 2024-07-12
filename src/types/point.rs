use std::ops::{Add, Div, Mul, Sub};

use rapier2d::na::Point2;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl From<Point2<f32>> for Point {
    fn from(value: Point2<f32>) -> Self {
        Self { x: value.x, y: value.y }
    }
}

impl From<Point> for Point2<f32> {
    fn from(value: Point) -> Self {
        Self::new(value.x, value.y)
    }
}
