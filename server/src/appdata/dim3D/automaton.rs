use std::thread;
use std::sync::{Arc, Mutex};

use rand::prelude::*;
use super::grid::CAGrid3D;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CellularAutomaton3D {
    prev_generation: CAGrid3D,
    curr_generation: CAGrid3D,
    dc_range: f32,
    dc_influence: f32,
    uc_range: f32,
    uc_influence: f32
}

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

    pub fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) {
        self.prev_generation = CAGrid3D::new(size);
        self.curr_generation = CAGrid3D::new(size);
        self.dc_range = dc_range;
        self.dc_influence = dc_influence;
        self.uc_range = uc_range;
        self.uc_influence = uc_influence;
    }

    pub fn clear_all_voxels(&mut self) {
        for x in 0..self.curr_generation.size() {
            for y in 0..self.curr_generation.size() {
                for z in 0..self.curr_generation.size() {
                    self.prev_generation.set(x, y, z, 0);
                    self.curr_generation.set(x, y, z, 0);
                }
            }
        }
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

    

    pub fn spread_chemicals_randomly(&mut self, chem: u32) {
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
    }
    
    fn total_influence(&self, px: usize, py: usize, pz: usize) -> f32 {
        let mut sum: f32 = 0.0;
    
        // Compute the boundaries of the proximity
        let max_dist = f32::ceil(f32::max(self.dc_range, self.uc_range)) as isize + 1;
    
        let xmin = isize::max(0, (px as isize) - max_dist) as usize;
        let ymin = isize::max(0, (py as isize) - max_dist) as usize;
        let zmin = isize::max(0, (pz as isize) - max_dist) as usize;
        let xmax = isize::min(self.prev_generation.size() as isize, (px as isize) + max_dist) as usize;
        let ymax = isize::min(self.prev_generation.size() as isize, (py as isize) + max_dist) as usize;
        let zmax = isize::min(self.prev_generation.size() as isize, (pz as isize) + max_dist) as usize;
    
        for x in xmin..xmax {
            for y in ymin..ymax {
                for z in zmin..zmax {
                
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

    fn start_thread(automaton: CellularAutomaton3D, computed_influences: Arc<Mutex<Vec<Vec<Vec<f32>>>>>, xmin: usize, xmax: usize) -> std::thread::JoinHandle<()> {
        let handle = thread::spawn(move || {
            // Multiprocessing

            for x in xmin..xmax {
                for y in 0..automaton.size() {
                    for z in 0..automaton.size() {
                        let influence = automaton.total_influence(x, y, z);

                        let mut computed_influences_locked = computed_influences.lock().unwrap();
                        computed_influences_locked[x][y][z] = influence;
                        drop(computed_influences_locked);
                    }
                }
            }

        });

        handle
    }

    fn run_iteration_helper(&self) -> Vec<Vec<Vec<f32>>> {
        let size: usize = self.size();

        const NUM_THREADS: usize = 16;
        let num_x_per_thread: usize = size / NUM_THREADS;

        let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
        let computed_influences: Arc<Mutex<Vec<Vec<Vec<f32>>>>> = Arc::new(Mutex::new(vec![vec![vec![0f32; size]; size]; size]));

        for t in 0..NUM_THREADS {

            let xmin = t * num_x_per_thread;
            let mut xmax = (t + 1) * num_x_per_thread;

            if t == NUM_THREADS - 1 {
                xmax = size;
            }

            let computed_influences_clone = computed_influences.clone();

            handles.push(
                CellularAutomaton3D::start_thread(self.clone(), computed_influences_clone, xmin, xmax)
            );
        }

        // Join all threads again
        for h in handles {
            h.join().unwrap();
        }

        // Extract the resulting vector
        let influence_results = computed_influences.lock().unwrap();

        influence_results.to_vec()
    }

    pub fn run_iteration(&mut self) {
        // The current generation becomes the previous one
        // and we're going to render the new generation here.
        let size: usize = self.size();
        self.prev_generation = self.curr_generation.clone();
        self.curr_generation = CAGrid3D::new(size);


        // Step 1: Computing influences for every point in the grid
        // Array of thread handles
        let influence_results = self.run_iteration_helper();

        // Step 2: Using this influence to change the state of a voxel
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                
                    let influence = influence_results[x][y][z];
        
                    if influence > 0.0 {
                        self.curr_generation.set(x, y, z, 0);
                    } else if influence < 0.0 {
                        self.curr_generation.set(x, y, z, 1);
                    }

                }
            }
        }
    }

}