use crate::{
    exports::{Edges, TokenShape},
    nodes::GridlessNode,
    traits::{AStar, BaseGrid, Value},
    types::Point,
};
use wasm_bindgen::JsValue;

pub struct GridlessGrid {
    pub size: i32,
}

impl GridlessGrid {
    pub fn new(value: JsValue) -> Self {
        let size = value.get_value("size");

        Self { size }
    }
}

impl BaseGrid<GridlessNode> for GridlessGrid {
    fn get_adjacent_nodes(&self, node: &GridlessNode, end_node: &GridlessNode, edges: &Edges, offset: Point) -> Vec<(GridlessNode, u32)> {
        let _ = node;
        let _ = end_node;
        let _ = edges;
        let _ = offset;

        Vec::new()
    }

    fn get_center_point(&self, GridlessNode { i, j }: &GridlessNode) -> Point {
        let half_size = (self.size as f32) / 2.0;
        Point { x: ((j * self.size) as f32) + half_size, y: ((i * self.size) as f32) + half_size }
    }

    fn get_direct_path(&self, start: &GridlessNode, end: &GridlessNode) -> Vec<GridlessNode> {
        let _ = start;
        let _ = end;

        Vec::new()
    }

    fn get_node(&self, point: Point) -> GridlessNode {
        let size = self.size as f32;
        GridlessNode { i: (point.y / size).floor() as i32, j: (point.x / size).floor() as i32 }
    }

    fn get_token_shape(&self, token: JsValue) -> TokenShape {
        let token_width: f32;
        let token_height: f32;

        if token.is_object() {
            token_width = token.get("document").get_value("width");
            token_height = token.get("document").get_value("height");
        } else {
            token_width = 1.0;
            token_height = 1.0;
        }

        let width = token_width * (self.size as f32);
        let height = token_height * (self.size as f32);

        TokenShape {
            center: Point { x: 0.0, y: 0.0 },
            offset: Point { x: 0.0, y: 0.0 },
            points: vec![
                Point { x: -(width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: (height / 2.0) },
                Point { x: -(width / 2.0), y: (height / 2.0) },
            ],
        }
    }
}

impl AStar for GridlessGrid {
    fn find_path(&self, path: Vec<Point>, goal: Point, offset: Point, edges: &Edges) -> Vec<Point> {
        let _ = path;
        let _ = goal;
        let _ = offset;
        let _ = edges;

        Vec::new()
    }
}
