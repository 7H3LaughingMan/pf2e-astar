use crate::{
    exports::{Edges, TokenShape},
    nodes::SquareNode,
    traits::{AStar, BaseGrid, Node, Value},
    types::{Overflow, Point},
};
use wasm_bindgen::JsValue;

pub struct SquareGrid {
    pub size: i32,
}

impl SquareGrid {
    pub fn new(value: JsValue) -> Self {
        let size = value.get_value("size");

        Self { size }
    }
}

impl BaseGrid<SquareNode> for SquareGrid {
    fn get_adjacent_nodes(&self, node: &SquareNode, end_node: &SquareNode, edges: &Edges, offset: Point) -> Vec<(SquareNode, u32)> {
        let neighbors = node.get_neighbors();

        let path = self.get_direct_path(node, end_node);
        let next_neighbor = if path.len() > 1 { path[1] } else { path[0] };
        let index = neighbors.iter().position(|(node, _)| node.at_node(&next_neighbor)).unwrap_or(0);
        let i = Overflow { value: index, limit: 7 };

        let neighbors = vec![
            neighbors[i.value],
            neighbors[(i + 4).value],
            neighbors[(i + 3).value],
            neighbors[(i + 5).value],
            neighbors[(i + 2).value],
            neighbors[(i + 6).value],
            neighbors[(i + 1).value],
            neighbors[(i + 7).value],
        ];

        neighbors
            .into_iter()
            .filter(|(neighbor, _cost)| !edges.check_collision(self.get_center_point(node) + offset, self.get_center_point(neighbor) + offset))
            .collect()
    }

    fn get_center_point(&self, SquareNode { i, j, d: _ }: &SquareNode) -> Point {
        let half_size = (self.size as f32) / 2.0;
        Point { x: ((j * self.size) as f32) + half_size, y: ((i * self.size) as f32) + half_size }
    }

    fn get_direct_path(&self, start: &SquareNode, end: &SquareNode) -> Vec<SquareNode> {
        let SquareNode { i: mut i0, j: mut j0, d: _ } = *start;
        let SquareNode { i: i1, j: j1, d: _ } = *end;
        let mut path = vec![*start];

        if i0 == i1 && j0 == j1 {
            return path;
        }

        let di = (i0 - i1).abs();
        let dj = 0 - (j0 - j1).abs();
        let si = if i0 < i1 { 1 } else { -1 };
        let sj = if j0 < j1 { 1 } else { -1 };
        let mut e = di + dj;

        loop {
            let e2 = e * 2;
            if e2 >= dj {
                e += dj;
                i0 += si;
            }
            if e2 <= di {
                e += di;
                j0 += sj;
            }
            if i0 == i1 && j0 == j1 {
                break;
            }
            path.push(SquareNode { i: i0, j: j0, d: false });
        }
        path.push(SquareNode { i: i1, j: j1, d: false });

        for idx in 1..path.len() {
            let (left, right) = path.split_at_mut(idx);
            right[0].from(&left[idx - 1]);
        }

        path
    }

    fn get_node(&self, point: Point) -> SquareNode {
        let size = self.size as f32;
        SquareNode { i: (point.y / size).floor() as i32, j: (point.x / size).floor() as i32, d: false }
    }

    fn get_token_shape(&self, token: JsValue) -> TokenShape {
        let token_width: f32;
        let token_height: f32;

        if token.is_object() {
            token_width = f32::max(token.get("document").get_value("width"), 1.0);
            token_height = f32::max(token.get("document").get_value("height"), 1.0);
        } else {
            token_width = 1.0;
            token_height = 1.0;
        }

        let width = token_width * (self.size as f32);
        let height = token_height * (self.size as f32);

        let offset =
            if token_width % 2.0 != 1.0 { Point { x: (self.size as f32) / 2.0, y: (self.size as f32) / 2.0 } } else { Point { x: 0.0, y: 0.0 } };

        TokenShape {
            center: Point { x: 0.0, y: 0.0 },
            offset,
            points: vec![
                Point { x: -(width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: (height / 2.0) },
                Point { x: -(width / 2.0), y: (height / 2.0) },
            ],
        }
    }
}

impl AStar for SquareGrid {
    fn find_path(&self, path: Vec<Point>, goal: Point, offset: Point, edges: &Edges) -> Vec<Point> {
        let mut path: Vec<SquareNode> = path.into_iter().map(|point| self.get_node(point - offset)).collect();

        if path.is_empty() {
            return Vec::new();
        }

        for idx in 1..path.len() {
            let (left, right) = path.split_at_mut(idx);
            right[0].from(&left[idx - 1]);
        }

        let start_node = *path.last().unwrap();
        let end_node = self.get_node(goal - offset);

        if start_node.at_node(&end_node) {
            return Vec::new();
        }

        let result = pathfinding::prelude::astar(
            &start_node,
            |node| self.get_adjacent_nodes(node, &end_node, edges, offset),
            |node| node.get_distance(&end_node),
            |node| node.at_node(&end_node),
        );

        if let Some((nodes, _cost)) = result {
            return nodes.iter().map(|node| self.get_center_point(node) + offset).collect();
        }

        Vec::new()
    }
}
