use wasm_bindgen::prelude::*;
use rand::prelude::*;

use super::grids::*;
use super::chemicals::*;

#[wasm_bindgen]
pub struct CellularAutomaton2D {
    grid: CAGrid2D,
    dc_range: f32,
    dc_influence: f32,
    uc_range: f32,
    uc_influence: f32
}

#[wasm_bindgen]
impl CellularAutomaton2D {

    pub fn new(size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) -> Self {
        CellularAutomaton2D {
            grid: CAGrid2D::new(size),
            dc_range,
            dc_influence,
            uc_range,
            uc_influence
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.grid.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, val: u32) {
        self.grid.set(x, y, val);
    }

    pub fn size(&self) -> usize {
        self.grid.size()
    }

    fn total_influence(&self, px: usize, py: usize) -> f32 {
        let mut sum: f32 = 0.0;

        for x in 0..self.grid.size() {
            for y in 0..self.grid.size() {
                
                if self.grid.get(x, y) == 0 {
                    let dist = CAGrid2D::dist(px, py, x, y);

                    if dist <= self.dc_range {
                        sum += self.dc_influence;
                    } else if dist <= self.uc_range {
                        sum += self.uc_influence;
                    }
                }

            }
        }

        sum
    }

    pub fn spread_chemicals_randomly(mut self, chem: u32) -> Self {
        // Random number generator
        let mut rng = rand::thread_rng();

        // Loop over all the cells in the grid
        for x in 0..self.grid.size() {
            for y in 0..self.grid.size() {
                self.grid.set(x, y, rng.gen_range(0..chem));
            }
        }

        self
    }

    pub fn run_iteration(mut self) -> Self {
        for x in 0..self.grid.size() {
            for y in 0..self.grid.size() {
                
                let influence = self.total_influence(x, y);
    
                if influence > 0.0 {
                    self.grid.set(x, y, 0);
                } else if influence < 0.0 {
                    self.grid.set(x, y, 1);
                }
    
            }
        }

        self
    }

}








#[wasm_bindgen]
pub struct CellularAutomaton3D {
    prev_generation: CAGrid3D,
    curr_generation: CAGrid3D,
    dc_range: f32,
    dc_influence: f32,
    uc_range: f32,
    uc_influence: f32
}

#[wasm_bindgen]
impl CellularAutomaton3D {

    pub fn new(size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) -> Self {
        CellularAutomaton3D {
            prev_generation: CAGrid3D::new(size),
            curr_generation: CAGrid3D::new(size),
            dc_range,
            dc_influence,
            uc_range,
            uc_influence
        }
    }

    pub fn reset_all_voxels(mut self) -> Self {
        for x in 0..self.curr_generation.size() {
            for y in 0..self.curr_generation.size() {
                for z in 0..self.curr_generation.size() {
                    self.prev_generation.set(x, y, z, 0);
                    self.curr_generation.set(x, y, z, 0);
                }
            }
        }

        self
    }

    pub fn get_dc_range(&self) -> f32 {
        self.dc_range
    }

    pub fn get_dc_influence(&self) -> f32 {
        self.dc_influence
    }

    pub fn get_uc_range(&self) -> f32 {
        self.uc_range
    }

    pub fn get_uc_influence(&self) -> f32 {
        self.uc_influence
    }

    pub fn set_dc_range(mut self, dc_range: f32) -> Self {
        self.dc_range = dc_range;

        self
    }

    pub fn set_dc_influence(mut self, dc_influence: f32) -> Self {
        self.dc_influence = dc_influence;

        self
    }

    pub fn set_uc_range(mut self, uc_range: f32) -> Self {
        self.uc_range = uc_range;

        self
    }

    pub fn set_uc_influence(mut self, uc_influence: f32) -> Self {
        self.uc_influence = uc_influence;

        self
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.curr_generation.get(x, y, z)
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u32) {
        self.prev_generation.set(x, y, z, val);
        self.curr_generation.set(x, y, z, val);
    }

    pub fn size(&self) -> usize {
        self.curr_generation.size()
    }

    fn total_influence(&self, px: usize, py: usize, pz: usize) -> f32 {
        let mut sum: f32 = 0.0;

        for x in 0..self.prev_generation.size() {
            for y in 0..self.prev_generation.size() {
                for z in 0..self.prev_generation.size() {
                
                    if self.prev_generation.get(x, y, z) == 0
                        && !(px == x && py == y && pz == z) {
                        let dist = CAGrid3D::dist(px, py, pz, x, y, z);

                        if dist <= self.dc_range {
                            sum += self.dc_influence;
                        } else if dist <= self.uc_range {
                            sum += self.uc_influence;
                        }
                    }

                }
            }
        }

        sum
    }

    pub fn spread_chemicals_randomly(mut self, chem: u32) -> Self {
        // Random number generator
        let mut rng = rand::thread_rng();

        // Loop over all the cells in the grid
        for x in 0..self.curr_generation.size() {
            for y in 0..self.curr_generation.size() {
                for z in 0..self.curr_generation.size() {
                    self.set(x, y, z, rng.gen_range(0..chem));
                }
            }
        }

        self
    }

    pub fn run_iteration(mut self) -> Self {
        // The current generation becomes the previous one
        // and we're going to render the new generation here.
        let size = self.size();
        self.prev_generation = self.curr_generation;
        self.curr_generation = CAGrid3D::new(size);

        for x in 0..self.curr_generation.size() {
            for y in 0..self.curr_generation.size() {
                for z in 0..self.curr_generation.size() {
                
                    let influence = self.total_influence(x, y, z);
        
                    if influence > 0.0 {
                        self.curr_generation.set(x, y, z, 0);
                    } else if influence < 0.0 {
                        self.curr_generation.set(x, y, z, 1);
                    }

                }
            }
        }

        self
    }

}