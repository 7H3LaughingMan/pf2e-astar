use crate::{traits::Node, types::Overflow};
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

    pub fn from(&mut self, origin: &SquareNode) {
        let di = (origin.i - self.i).abs();
        let dj = (origin.j - self.j).abs();
        let nd = i32::min(di, dj) + if origin.d { 1 } else { 0 };

        self.d = (nd & 1) == 1;
    }

    pub fn get_next_direct_node(&self, end_node: &SquareNode) -> SquareNode {
        let SquareNode { i: mut i0, j: mut j0, d: _ } = *self;
        let SquareNode { i: i1, j: j1, d: _ } = *end_node;

        if i0 == i1 && j0 == j1 {
            return *end_node;
        }

        let di = (i0 - i1).abs();
        let dj = 0 - (j0 - j1).abs();
        let si = if i0 < i1 { 1 } else { -1 };
        let sj = if j0 < j1 { 1 } else { -1 };
        let e2 = (di + dj) * 2;

        if e2 >= dj {
            i0 += si;
        }

        if e2 <= di {
            j0 += sj;
        }

        SquareNode { i: i0, j: j0, d: false }
    }
}

impl Node for SquareNode {
    fn at_node(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }

    fn get_distance(&self, other: &Self) -> u32 {
        let di = (self.i - other.i).abs();
        let dj = (self.j - other.j).abs();

        let ns = (di - dj).abs();
        let nd = i32::min(di, dj);
        let cd = if self.d { ((nd + 1) & -2) + (nd >> 1) } else { (nd & -2) + ((nd + 1) >> 1) };

        (ns + cd) as u32
    }

    fn get_neighbors(&self, end_node: &Self) -> Vec<(Self, u32)> {
        let SquareNode { i, j, d } = *self;

        let neighbors = [
            (SquareNode::new(i, j - 1, d), 1),
            (SquareNode::new(i + 1, j - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j, d), 1),
            (SquareNode::new(i + 1, j + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i, j + 1, d), 1),
            (SquareNode::new(i - 1, j + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j, d), 1),
            (SquareNode::new(i - 1, j - 1, !d), if d { 2 } else { 1 }),
        ];

        let next_neighbor = self.get_next_direct_node(end_node);
        let idx = neighbors.iter().position(|(node, _)| node.at_node(&next_neighbor)).unwrap_or(0);
        let i = Overflow { value: idx, limit: 7 };

        vec![
            neighbors[i.value],
            neighbors[(i + 4).value],
            neighbors[(i + 3).value],
            neighbors[(i + 5).value],
            neighbors[(i + 2).value],
            neighbors[(i + 6).value],
            neighbors[(i + 1).value],
            neighbors[(i + 7).value],
        ]
    }
}
