use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CAGrid2D {
    size: usize,
    data: Vec<Vec<u32>>
}

#[wasm_bindgen]
impl CAGrid2D {
    pub fn new(size: usize) -> CAGrid2D {
        CAGrid2D {
            size: size,
            data: vec![vec![0; size]; size]
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.data[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, val: u32) {
        self.data[x][y] = val;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}



#[wasm_bindgen]
pub struct CAGrid3D {
    size: usize,
    data: Vec<Vec<Vec<u32>>>
}

#[wasm_bindgen]
impl CAGrid3D {
    pub fn new(size: usize) -> CAGrid3D {
        CAGrid3D {
            size: size,
            data: vec![vec![vec![0; size]; size]; size]
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.data[x][y][z]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u32) {
        self.data[x][y][z] = val;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
