use crate::{exports::Edges, types::Point};
use rapier2d::{na, prelude::{Ray, *}};

pub trait AStar {
    fn find_path(&self, start: Point, end: Point, offset: Point, edges: &Edges) -> Vec<Point>;
}