use crate::{
    exports::{Edges, TokenShape},
    traits::Node,
    types::Point,
};
use wasm_bindgen::JsValue;

pub trait BaseGrid<T: Node> {
    fn get_adjacent_nodes(&self, node: &T, end_node: &T, edges: &Edges, offset: Point) -> Vec<(T, u32)>;
    fn get_center_point(&self, node: &T) -> Point;
    fn get_node(&self, point: Point) -> T;
    fn get_token_shape(&self, token: JsValue) -> TokenShape;
}
