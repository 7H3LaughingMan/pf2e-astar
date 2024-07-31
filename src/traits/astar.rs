use crate::{exports::Edges, types::Point};

pub trait AStar {
    fn find_path(&self, path: Vec<Point>, goal: Point, offset: Point, edges: &Edges) -> Vec<Point>;
}
