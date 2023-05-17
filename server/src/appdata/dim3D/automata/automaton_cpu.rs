use std::thread;
use std::sync::{Arc, Mutex};

use rand::prelude::*;
use super::super::grid::CAGrid3D;
use super::automaton::CellularAutomaton3D;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CPUCellularAutomaton3D {
    pub prev_generation: CAGrid3D,
    pub curr_generation: CAGrid3D,
    pub dc_range: f32,
    pub dc_influence: f32,
    pub uc_range: f32,
    pub uc_influence: f32,
    iteration_count: u32
}

impl CPUCellularAutomaton3D {

    pub fn new(size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) -> Self {
        CPUCellularAutomaton3D {
            prev_generation: CAGrid3D::new(size),
            curr_generation: CAGrid3D::new(size),
            dc_range,
            dc_influence,
            uc_range,
            uc_influence,
            iteration_count: 0
        }
    }

    fn total_influence(&self, px: usize, py: usize, pz: usize) -> f32 {
        let mut sum: f32 = 0.0;

        let size_i = self.size() as i32;

        // UC has a larger range than DC, so pull it up to the closest larger integer and use it as range
        let uc_range = f32::ceil(self.uc_range) as i32 + 1; // +1 as the x..y excludes y

        for x in -uc_range..uc_range {
            for y in -uc_range..uc_range {
                for z in -uc_range..uc_range {
                    // Compute the distance from the point (0, 0, 0)
                    let dist = f32::sqrt((x*x + y*y + z*z) as f32);

                    // Calculate the (x,y,z) coordinates when they wrap around the cube
                    // in either x, y or z-direction

                    // 1. Calculate the signed index, obtained when you sum (px, py, pz) with (x, y, z)
                    let mut x_wrapped = (px as i32) + x;
                    let mut y_wrapped = (py as i32) + y;
                    let mut z_wrapped = (pz as i32) + z;

                    // WRAPPING X
                    // If smaller than zero
                    if x_wrapped < 0 {
                        // Adjust so that it wraps around the cube
                        x_wrapped = size_i + x_wrapped;
                    }

                    // If larger than or equal to array-size
                    if x_wrapped >= size_i {
                        // Adjust so that it wraps around the cube
                        x_wrapped = x_wrapped - size_i;
                    }

                    // WRAPPING Y
                    // If smaller than zero
                    if y_wrapped < 0 {
                        // Adjust so that it wraps around the cube
                        y_wrapped = size_i + y_wrapped;
                    }

                    // If larger than or equal to array-size
                    if y_wrapped >= size_i {
                        // Adjust so that it wraps around the cube
                        y_wrapped = y_wrapped - size_i;
                    }

                    // WRAPPING Z
                    // If smaller than zero
                    if z_wrapped < 0 {
                        // Adjust so that it wraps around the cube
                        z_wrapped = size_i + z_wrapped;
                    }

                    // If larger than or equal to array-size
                    if z_wrapped >= size_i {
                        // Adjust so that it wraps around the cube
                        z_wrapped = z_wrapped - size_i;
                    }

                    // A voxel cannot be its own neighbour, so (0, 0, 0) must be excluded from the neighbour-pack
                    if self.prev_generation.get(x_wrapped as usize, y_wrapped as usize, z_wrapped as usize) == 0
                        && !(x == 0 && y == 0 && z == 0) {
                        // If this point falls within the range of Differentiated Cells
                        if dist <= self.dc_range {
                            // Add the DC-influence to the sum
                            sum += self.dc_influence;
                            sum -= self.uc_influence;
                        }

                        // Else: if this point falls within the range of Undifferentiated Cells
                        if dist <= self.uc_range {
                            // Add the UC-influence to the sum
                            sum += self.uc_influence;
                        }
                    }
                    
                }
            }
        }
    
        sum
    }

    fn start_thread(automaton: CPUCellularAutomaton3D, computed_influences: Arc<Mutex<Vec<Vec<Vec<f32>>>>>, xmin: usize, xmax: usize) -> std::thread::JoinHandle<()> {
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
                CPUCellularAutomaton3D::start_thread(self.clone(), computed_influences_clone, xmin, xmax)
            );
        }

        // Join all threads again
        for h in handles {
            h.join().unwrap();
        }

        // Extract the resulting vector
        let influence_results = computed_influences.lock().unwrap();

        let result = influence_results.to_vec();

        drop(influence_results);

        result
    }

}



impl CellularAutomaton3D for CPUCellularAutomaton3D {

    fn clear_all_voxels(&mut self) {
        for x in 0..self.curr_generation.size() {
            for y in 0..self.curr_generation.size() {
                for z in 0..self.curr_generation.size() {
                    self.prev_generation.set(x, y, z, 0);
                    self.curr_generation.set(x, y, z, 0);
                }
            }
        }

        // Reset the iteration count
        self.iteration_count = 0;
    }

    fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) {
        self.prev_generation = CAGrid3D::new(size);
        self.curr_generation = CAGrid3D::new(size);
        self.dc_range = dc_range;
        self.dc_influence = dc_influence;
        self.uc_range = uc_range;
        self.uc_influence = uc_influence;
    }

    fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.curr_generation.get(x, y, z)
    }

    fn set(&mut self, x: usize, y: usize, z: usize, val: u32) {
        self.prev_generation.set(x, y, z, val);
        self.curr_generation.set(x, y, z, val);
    }

    fn size(&self) -> usize {
        self.curr_generation.size()
    }

    fn spread_chemicals_randomly(&mut self, chem: u32) {
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

        // Reset the iteration count
        self.iteration_count = 0;
    }

    fn run_iteration(&mut self) {
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

                    // if !(px == 25 && py == 0 && pz == 0) {
                    //     continue;
                    // }
                
                    let influence = influence_results[x][y][z];
        
                    if influence > 0.0 {
                        self.curr_generation.set(x, y, z, 0);
                    } else if influence < 0.0 {
                        self.curr_generation.set(x, y, z, 1);
                    } else {
                        self.curr_generation.set(x, y, z, self.prev_generation.get(x, y, z));
                    }


                    // DEBUGGING CODE
                    // let size_i = self.size() as i32;

                    // // UC has a larger range than DC, so pull it up to the closest larger integer and use it as range
                    // let uc_range = f32::ceil(self.uc_range) as i32 + 1; // +1 as the x..y excludes y

                    // for x in -uc_range..uc_range {
                    //     for y in -uc_range..uc_range {
                    //         for z in -uc_range..uc_range {
                    //             // Compute the distance from the point (0, 0, 0)
                    //             let dist = f32::sqrt((x*x + y*y + z*z) as f32);

                    //             // Calculate the (x,y,z) coordinates when they wrap around the cube
                    //             // in either x, y or z-direction

                    //             // 1. Calculate the signed index, obtained when you sum (px, py, pz) with (x, y, z)
                    //             let mut x_wrapped = (px as i32) + x;
                    //             let mut y_wrapped = (py as i32) + y;
                    //             let mut z_wrapped = (pz as i32) + z;

                    //             // WRAPPING X
                    //             // If smaller than zero
                    //             if x_wrapped < 0 {
                    //                 // Adjust so that it wraps around the cube
                    //                 x_wrapped = size_i + x_wrapped;
                    //             }

                    //             // If larger than or equal to array-size
                    //             if x_wrapped >= size_i {
                    //                 // Adjust so that it wraps around the cube
                    //                 x_wrapped = x_wrapped - size_i;
                    //             }

                    //             // WRAPPING Y
                    //             // If smaller than zero
                    //             if y_wrapped < 0 {
                    //                 // Adjust so that it wraps around the cube
                    //                 y_wrapped = size_i + y_wrapped;
                    //             }

                    //             // If larger than or equal to array-size
                    //             if y_wrapped >= size_i {
                    //                 // Adjust so that it wraps around the cube
                    //                 y_wrapped = y_wrapped - size_i;
                    //             }

                    //             // WRAPPING Z
                    //             // If smaller than zero
                    //             if z_wrapped < 0 {
                    //                 // Adjust so that it wraps around the cube
                    //                 z_wrapped = size_i + z_wrapped;
                    //             }

                    //             // If larger than or equal to array-size
                    //             if z_wrapped >= size_i {
                    //                 // Adjust so that it wraps around the cube
                    //                 z_wrapped = z_wrapped - size_i;
                    //             }

                    //             // A voxel cannot be its own neighbour, so (0, 0, 0) must be excluded from the neighbour-pack
                    //             if self.prev_generation.get(x_wrapped as usize, y_wrapped as usize, z_wrapped as usize) == 0
                    //                 && !(x == 0 && y == 0 && z == 0) {
                    //                 // If this point falls within the range of Differentiated Cells
                    //                 if dist <= self.dc_range {
                    //                     // Add the DC-influence to the sum
                    //                     self.curr_generation.set(x_wrapped as usize, y_wrapped as usize, z_wrapped as usize, 1);
                    //                 }

                    //                 // Else: if this point falls within the range of Undifferentiated Cells
                    //                 if dist <= self.uc_range {
                    //                     // Add the UC-influence to the sum
                    //                     self.curr_generation.set(x_wrapped as usize, y_wrapped as usize, z_wrapped as usize, 1);
                    //                 }
                    //             }
                                
                    //         }
                    //     }
                    // }

                    // END OF DEBUGGING CODE



                }
            }
        }

        self.iteration_count += 1;

    }

    fn set_iteration_count(&mut self, iterations: u32) {
        self.iteration_count = iterations;
    }

    fn get_iteration_count(&self) -> u32 {
        self.iteration_count
    }

}