use crate::traits::Node;
use std::hash::Hash;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct HexagonalNode {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexagonalNode {
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s }
    }

    pub fn round(q: f32, r: f32, s: f32) -> HexagonalNode {
        let mut iq = q.round();
        let mut ir = r.round();
        let mut is = s.round();
        let dq = (iq - q).abs();
        let dr = (ir - r).abs();
        let ds = (is - s).abs();

        if (dq > dr) && (dq > ds) {
            iq = -ir - is;
        } else if dr > ds {
            ir = -iq - iq;
        } else {
            is = -iq - ir;
        }

        HexagonalNode { q: iq as i32, r: ir as i32, s: is as i32 }
    }
}

impl Node for HexagonalNode {
    fn at_node(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r && self.s == other.s
    }

    fn get_distance(&self, other: &Self) -> u32 {
        let dq = self.q - other.q;
        let dr = self.r - other.r;

        ((dq.abs() + dr.abs() + (dq + dr).abs()) / 2) as u32
    }

    fn get_neighbors(&self) -> Vec<(Self, u32)> {
        let HexagonalNode { q, r, s } = *self;

        vec![
            (HexagonalNode::new(q, r - 1, s + 1), 1),
            (HexagonalNode::new(q + 1, r - 1, s), 1),
            (HexagonalNode::new(q + 1, r, s - 1), 1),
            (HexagonalNode::new(q, r + 1, s - 1), 1),
            (HexagonalNode::new(q - 1, r + 1, s), 1),
            (HexagonalNode::new(q - 1, r, s + 1), 1),
        ]
    }
}
