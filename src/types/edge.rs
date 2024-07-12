use rapier2d::prelude::{Collider, ColliderBuilder};

use super::{Point, Rectangle};

#[derive(Clone)]
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Edge {
    pub a: Point,
    pub b: Point,
    pub bounds: Rectangle,
    pub direction: i32,
    pub id: String,
    pub r#move: i32,
    pub r#type: String,
}

impl From<Edge> for Collider {
    fn from(value: Edge) -> Self {
        ColliderBuilder::polyline(vec![value.a.into(), value.b.into()], None).build()
    }
}
