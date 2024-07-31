use super::Point;
use rapier2d::{na::Point2, prelude::Polyline};

#[derive(Clone)]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Polygon {
    pub center: Point,
    pub offset: Point,
    pub points: Vec<Point>,
}

impl Polygon {
    pub fn centroid(points: &Vec<Point>) -> Point {
        if points.is_empty() {
            return Point { x: 0_f32, y: 0_f32 };
        }

        let mut x = 0_f32;
        let mut y = 0_f32;
        let mut a = 0_f32;

        let Point { x: mut x0, y: mut y0 } = points[points.len() - 1];

        for Point { x: x1, y: y1 } in points {
            let z = (x0 * y1) - (x1 * y0);
            x += (x0 + x1) * z;
            y += (y0 + y1) * z;
            x0 = *x1;
            y0 = *y1;
            a += z;
        }

        a *= 3.0;
        x /= a;
        y /= a;

        Point { x, y }
    }

    pub fn shrink(&self, size: Point) -> Polygon {
        let mut points = Vec::new();

        for Point { x, y } in self.points.clone() {
            points.push(Point {
                x: if x < 0.0 {
                    x + size.x
                } else if x > 0.0 {
                    x - size.x
                } else {
                    0.0
                },
                y: if y < 0.0 {
                    y + size.y
                } else if x > 0.0 {
                    y - size.y
                } else {
                    0.0
                },
            });
        }

        Polygon { center: self.center, offset: self.offset, points }
    }

    fn get_vertices(&self) -> Vec<Point2<f32>> {
        let mut vertices = Vec::new();

        for Point { x, y } in self.points.clone() {
            vertices.push(Point2::<f32>::new(x, y));
        }

        vertices
    }

    fn get_indices(&self) -> Vec<[u32; 2]> {
        let mut indices = Vec::new();

        for i in 0..(self.points.len() as u32 - 1) {
            indices.push([i, i + 1]);
        }
        indices.push([self.points.len() as u32 - 1, 0]);

        indices
    }

    pub fn to_polyline(&self) -> Polyline {
        let vertices = self.get_vertices();
        let indices = Some(self.get_indices());

        Polyline::new(vertices, indices)
    }
}
