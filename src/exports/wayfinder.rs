use crate::{
    enums::Grid,
    exports::{Edges, Image},
    traits::AStar,
    types::{Pixel, Point, Polygon, Rectangle},
};
use rapier2d::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wayfinder {
    grid: Grid,
    edges: Edges,
    token_shape: Polygon,
}

const SHRINK: Point = Point { x: 0.40, y: 0.40 };

#[wasm_bindgen]
impl Wayfinder {
    #[wasm_bindgen(constructor)]
    pub fn new(token: JsValue) -> Wayfinder {
        let grid = Grid::new();
        let token_shape = grid.get_token_shape(token);
        let edges = Edges::new(token_shape.shrink(grid.get_size() * SHRINK).to_polyline());

        Wayfinder { grid, edges, token_shape }
    }

    #[wasm_bindgen(js_name = addExplored)]
    pub fn add_explored(&mut self, pixels: js_sys::Uint8Array, bounds: JsValue, scaled_bounds: JsValue) {
        let bounds: Rectangle = serde_wasm_bindgen::from_value(bounds).unwrap();
        let scaled_bounds: Rectangle = serde_wasm_bindgen::from_value(scaled_bounds).unwrap();
        let pixels: Vec<Pixel> = bytemuck::allocation::cast_vec(pixels.to_vec());
        
        self.edges.explored = Some(Image { pixels, bounds, scaled_bounds });
    }

    #[wasm_bindgen(js_name = findPath)]
    pub fn find_path(&self, start: JsValue, end: JsValue) -> JsValue {
        let start = serde_wasm_bindgen::from_value(start).unwrap();
        let end = serde_wasm_bindgen::from_value(end).unwrap();

        let path = match &self.grid {
            Grid::Gridless(gridless_grid) => gridless_grid.find_path(start, end, self.token_shape.offset, &self.edges),
            Grid::Square(square_grid) => square_grid.find_path(start, end, self.token_shape.offset, &self.edges),
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.find_path(start, end, self.token_shape.offset, &self.edges),
        };

        serde_wasm_bindgen::to_value(&Wayfinder::simplify_path(path)).unwrap()
    }

    fn simplify_path(nodes: Vec<Point>) -> Vec<Point> {
        let mut nodes = nodes.clone();
        let mut i = 0;

        while i + 2 < nodes.len() {
            let point_1: nalgebra::Point2<f32> = (nodes[i]).into();
            let point_2: nalgebra::Point2<f32> = (nodes[i + 1]).into();
            let point_3: nalgebra::Point2<f32> = (nodes[i + 2]).into();

            let ray_1 = Ray::new(point_1, point_2 - point_1);
            let ray_2 = Ray::new(point_2, point_3 - point_2);

            let angle = ray_1.dir.angle(&ray_2.dir);

            if -0.08727 < angle && angle < 0.08727 {
                nodes.remove(i + 1);
            } else {
                i += 1;
            }
        }

        nodes
    }
}
