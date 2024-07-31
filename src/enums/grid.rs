use wasm_bindgen::JsValue;

use crate::{
    grids::{gridless_grid, hexagonal_grid, GridlessGrid, HexagonalGrid, SquareGrid},
    traits::{BaseGrid, Value},
    types::{Point, Polygon},
    GAME,
};

pub enum Grid {
    Gridless(GridlessGrid),
    Square(SquareGrid),
    Hexagonal(HexagonalGrid),
}

impl Grid {
    pub fn new() -> Self {
        let grid = GAME.get("canvas").get("grid");
        let r#type: i32 = grid.get_value("type");

        match r#type {
            0 => Grid::Gridless(GridlessGrid::new(grid)),
            1 => Grid::Square(SquareGrid::new(grid)),
            2..=5 => Grid::Hexagonal(HexagonalGrid::new(grid)),
            type_ => panic!("Unknown Grid Type - {type_}"),
        }
    }

    pub fn get_token_shape(&self, token: JsValue) -> Polygon {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.get_token_shape(token),
            Grid::Square(square_grid) => square_grid.get_token_shape(token),
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.get_token_shape(token),
        }
    }

    pub fn get_size(&self) -> Point {
        match self {
            Grid::Gridless(gridless_grid) => Point { x: gridless_grid.size as f32, y: gridless_grid.size as f32 },
            Grid::Square(square_grid) => Point { x: square_grid.size as f32, y: square_grid.size as f32 },
            Grid::Hexagonal(hexagonal_grid) => Point { x: hexagonal_grid.size_x, y: hexagonal_grid.size_y },
        }
    }
}
