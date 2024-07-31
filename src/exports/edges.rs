use super::Image;
use crate::{
    traits::Value,
    types::{Edge, Point, Rectangle},
    GAME,
};
use rapier2d::{parry::query::ShapeCastOptions, prelude::*};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Edges {
    bounds: Rectangle,
    colliders: ColliderSet,
    edges: HashMap<ColliderHandle, Edge>,
    pipeline: QueryPipeline,
    rigid_bodies: RigidBodySet,
    shape: Polyline,
    pub explored: Option<Image>,
}

impl Edges {
    pub fn new(shape: Polyline) -> Self {
        let edges_map = js_sys::Map::from(GAME.get("canvas").get("edges"));

        let mut colliders = ColliderSet::new();
        let mut edges = HashMap::new();

        for edge in edges_map.values() {
            let edge: Edge = serde_wasm_bindgen::from_value(edge.unwrap()).unwrap();

            if edge.r#type == "wall" && edge.r#move == 20 {
                let handle = colliders.insert(edge.clone());
                edges.insert(handle, edge);
            }
        }

        let bounds: Rectangle = GAME.get("canvas").get("scene").get("dimensions").get_value("sceneRect");

        let mut pipeline = QueryPipeline::new();
        pipeline.update(&colliders);

        Edges { bounds, colliders, edges, pipeline, rigid_bodies: RigidBodySet::new(), shape, explored: None }
    }

    pub fn check_collision(&self, start: Point, end: Point) -> bool {
        if !self.bounds.contains(start) || !self.bounds.contains(end) {
            return true;
        }

        if let Some(explored) = &self.explored {
            if !explored.check_pixel(end) {
                return true;
            }
        }

        let shape_pos = Isometry::new(vector![start.x, start.y], 0.0);
        let shape_vel = vector![end.x - start.x, end.y - start.y];
        let options = ShapeCastOptions {
            max_time_of_impact: 1.0,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: false,
        };

        if let Some((_handle, _toi)) =
            self.pipeline.cast_shape(&self.rigid_bodies, &self.colliders, &shape_pos, &shape_vel, &self.shape, options, QueryFilter::default())
        {
            return true;
        }

        false
    }
}
