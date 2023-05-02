use wasm_bindgen::prelude::*;
use rand::prelude::*;
mod data;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn spread_chemicals_randomly_2d(mut grid: data::grids::CAGrid2D, chem: u32) -> data::grids::CAGrid2D {

    // Random number generator
    let mut rng = rand::thread_rng();

    // Loop over all the cells in the grid
    for x in 0..grid.size() {
        for y in 0..grid.size() {
            grid.set(x, y, rng.gen_range(0..chem));
        }
    }

    grid
}

#[wasm_bindgen]
pub fn run_iteration(mut grid: data::grids::CAGrid2D) -> data::grids::CAGrid2D {




    grid
}