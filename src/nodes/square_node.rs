use rapier2d::na::Point2;

use crate::traits::Node;
use std::hash::Hash;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct SquareNode {
    pub i: i32,
    pub j: i32,
    pub d: bool,
}

impl SquareNode {
    pub fn new(i: i32, j: i32, d: bool) -> Self {
        Self { i, j, d }
    }
}

impl Node for SquareNode {
    fn at_node(&self, other: &SquareNode) -> bool {
        self.i == other.i && self.j == other.j
    }

    fn get_distance(&self, other: &SquareNode) -> u32 {
        let di = (self.i - other.i).abs();
        let dj = (self.j - other.j).abs();

        let ns =(di - dj).abs();
        let nd = i32::min(di, dj);
        let cd = if self.d { ((nd + 1) & -2) + (nd >> 1) } else { (nd & -2) + ((nd + 1) >> 1) };

        (ns + cd) as u32
    }

    fn get_neighbors(&self) -> Vec<(Self, u32)> {
        let SquareNode { i, j, d } = *self;

        vec![
            (SquareNode::new(i, j - 1, d), 1),
            (SquareNode::new(i + 1, j - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j, d), 1),
            (SquareNode::new(i + 1, j + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i, j + 1, d), 1),
            (SquareNode::new(i - 1, j + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j, d), 1),
            (SquareNode::new(i - 1, j - 1, !d), if d { 2 } else { 1 }),
        ]
    }
}
