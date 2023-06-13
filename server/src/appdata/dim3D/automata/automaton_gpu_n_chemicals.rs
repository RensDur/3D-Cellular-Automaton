use super::automaton::CellularAutomaton3D;

use isosurface::{source::Source, marching_cubes::MarchingCubes};
use serde::{Serialize, Deserialize};

use rand::prelude::*;

use metal::*;
use objc::rc::autoreleasepool;
use std::mem;

use crate::{AUTOMATON_SIZE, routes::gpu_get, K_MAX};

const AUTOMATON_SHADER_SRC: &str = include_str!("automaton_n_chemicals_shader.metal");
const ORDER_PARAM_SHADER_SRC: &str = include_str!("order_param_n_chemicals_shader.metal");



//
// In order for this generalisation to work, a concept of a chemical must be structured.
//
#[derive(Clone, Serialize, Deserialize)]
pub struct CAChemical {
    // A chemical has a strength (influence) and a range
    pub range: f32,
    pub influence: f32
}


//
// Since chemicals are ordered into groups, there's a struct for that too
//
#[derive(Clone, Serialize, Deserialize)]
pub struct CAChemicalGroup {
    pub promote: CAChemical,
    pub demote: CAChemical
}



//
// This is the main struct that encapsulates the gpu-implementation of this generalisation
//
#[derive(Clone, Serialize, Deserialize)]
pub struct GPUNChemicalsCellularAutomaton3D {
    pub grid: Vec<Vec<Vec<u8>>>,
    pub chemicals: Vec<CAChemicalGroup>,
    iteration_count: u32,
    marching_cubes_chemical_capture: usize,
    order_parameter: Vec<Vec<f32>>
}




//
// Basis implementation of the GPUNChemicalsCellularAutomaton3D
//
impl GPUNChemicalsCellularAutomaton3D {

    pub fn new(chemicals: Vec<CAChemicalGroup>) -> Self {
        GPUNChemicalsCellularAutomaton3D {
            grid: vec![vec![vec![0u8; AUTOMATON_SIZE]; AUTOMATON_SIZE]; AUTOMATON_SIZE],
            chemicals,
            iteration_count: 0,
            marching_cubes_chemical_capture: 0,
            order_parameter: vec![]
        }
    }

    //
    // The capture functions can be used to alter which chemical should be captured by
    // the marching cubes algorithm.
    //

    pub fn capture_chemical(&mut self, chemical: usize) {
        self.marching_cubes_chemical_capture = chemical;
    }

    pub fn get_captured_chemical(&self) -> usize {
        self.marching_cubes_chemical_capture
    }

    pub fn insert_order_parameter_value(&mut self, val: Vec<f32>) {
        // println!("Inserting order parameter {}", val);

        self.order_parameter.push(val);
    }

    pub fn get_order_parameters(&self) -> Vec<Vec<f32>> {
        // In this class, the order parameters are organised as follows:
        // self.order_parameter is a Vec<Vec<f32>> and contains a Vec<f32> for every iteration.
        // Every iteration Vec<f32> contains (K+1) entries: K epsilons and the undif. epsilon
        // This is a good format for collecting order parameters along the way, but not for 
        // sharing them with other parts of the system.

        // Here, we'll therefore transform the order parameter into a Vec<Vec<f32>> that contains
        // a Vec<f32> for every (K+1) cell-types. Each Vec<f32> then contains one f32 for every iteration.

        let mut result: Vec<Vec<f32>> = vec![];

        // Add K+1 empty vectors
        for spec in 0..(self.chemicals.len() + 1) {
            result.push(vec![]);
        }

        // For each iteration
        for iter in 0..self.order_parameter.len() {
            // Go over all the species again
            for spec in 0..(self.chemicals.len() + 1) {
                // Append the next iteration for this species
                result[spec].push(self.order_parameter[iter][spec]);
            }
        }

        result
    }

    //
    // The import and export functions are exactly the same as the original gpu implementation
    //

    pub fn export(&self) -> Vec<u8> {

        let mut res = vec![0u8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE];

        for x in 0..AUTOMATON_SIZE {
            for y in 0..AUTOMATON_SIZE {
                for z in 0..AUTOMATON_SIZE {
                    res[x + y*AUTOMATON_SIZE + z*AUTOMATON_SIZE*AUTOMATON_SIZE] = self.grid[x][y][z];
                }
            }
        }

        res

    }

    fn import(&mut self, data: Vec<u8>) {

        for x in 0..AUTOMATON_SIZE {
            for y in 0..AUTOMATON_SIZE {
                for z in 0..AUTOMATON_SIZE {
                    self.grid[x][y][z] = data[x + y*AUTOMATON_SIZE + z*AUTOMATON_SIZE*AUTOMATON_SIZE];
                }
            }
        }

    }



    //
    // COMPUTING THE ORDER-PARAMETERS
    //

    fn compute_order_parameter(&mut self) {
        autoreleasepool(|| {

            let device = Device::system_default().expect("no device found");
            // println!("Computing order-parameter on GPU: {}", device.name());
            let command_queue = device.new_command_queue();

            let data = self.export();

            let buffer = device.new_buffer_with_data(
                unsafe { mem::transmute(data.as_ptr()) },
                (data.len() * mem::size_of::<u8>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            );

            let sum = {
                let data: Vec<i8> = vec![0i8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE * (K_MAX+1)];
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<i8>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache,
                )
            };

            // There's a couple of size parameters we need to pass over to the gpu
            let size_container: Vec<u32> = vec![AUTOMATON_SIZE as u32, self.chemicals.len() as u32 + 1];

            let arg_size_container = {
                let data = size_container.as_slice();
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<u32>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache
                )
            };

            // There should lastly be a buffer that records the neighbours
            let neighbours: Vec<i32> = vec![-1, 0, 0,
                                            0, -1, 0,
                                            0, 0, -1,
                                            1, 0, 0,
                                            0, 1, 0,
                                            0, 0, 1];

            let arg_neighbours = {
                let data = neighbours.as_slice();
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<i32>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache
                )
            };

            let command_buffer = command_queue.new_command_buffer();
            let encoder = command_buffer.new_compute_command_encoder();

            let library = device
                .new_library_with_source(ORDER_PARAM_SHADER_SRC, &CompileOptions::new())
                .unwrap();
            let kernel = library.get_function("compute_iteration", None).unwrap();

            let argument_encoder = kernel.new_argument_encoder(0);
            let arg_buffer = device.new_buffer(
                argument_encoder.encoded_length(),
                MTLResourceOptions::empty(),
            );
            argument_encoder.set_argument_buffer(&arg_buffer, 0);
            argument_encoder.set_buffer(0, &buffer, 0);
            argument_encoder.set_buffer(1, &sum, 0);
            argument_encoder.set_buffer(2, &arg_size_container, 0);
            argument_encoder.set_buffer(3, &arg_neighbours, 0);

            let pipeline_state_descriptor = ComputePipelineDescriptor::new();
            pipeline_state_descriptor.set_compute_function(Some(&kernel));

            let pipeline_state = device
                .new_compute_pipeline_state_with_function(
                    pipeline_state_descriptor.compute_function().unwrap(),
                )
                .unwrap();

            encoder.set_compute_pipeline_state(&pipeline_state);
            encoder.set_buffer(0, Some(&arg_buffer), 0);

            encoder.use_resource(&buffer, MTLResourceUsage::Read);
            encoder.use_resource(&sum, MTLResourceUsage::Write);
            encoder.use_resource(&arg_size_container, MTLResourceUsage::Read);
            encoder.use_resource(&arg_neighbours, MTLResourceUsage::Read);

            let width = pipeline_state.thread_execution_width();
            let height = pipeline_state.max_total_threads_per_threadgroup() / width;

            let threads_per_grid = MTLSize {
                width: (data.len() as u64),
                height: 1,
                depth: 1,
            };

            let threads_per_thread_group = MTLSize {
                width,
                height,
                depth: 1,
            };

            encoder.dispatch_threads(threads_per_grid, threads_per_thread_group);
            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();





            let mut result: Vec<f32> = vec![];
            let result_cell_sums: Vec<i8>;

            // Define the normalisation constant
            let normalisation = 6.0 * (AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE) as f32;

            // Extract the obtained sums in the 'result_cell_sums' container
            let ptr = sum.contents() as *mut [i8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE * (K_MAX+1)];
            unsafe {
                result_cell_sums = (*ptr).to_vec();
            }

            for spec in 0..(self.chemicals.len()+1) {
                // Push a new f32 into the array
                result.push(0.0);

                // Now, for each cell in the CA, add all values of this species
                for i in 0..(AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE) {
                    result[spec] += result_cell_sums[(self.chemicals.len()+1)*i + spec] as f32 / normalisation;
                }
            }

            self.insert_order_parameter_value(result);

        });

    }



}


//
// Implement the CellularAutomaton3D contract for this automaton
//
impl CellularAutomaton3D for GPUNChemicalsCellularAutomaton3D {

    // Clearing all the voxels is done in exactly the same way as in the original gpu implementation.
    // All entries are set to zero and the iteration count to zero.
    fn clear_all_voxels(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                for z in 0..self.grid.len() {
                    self.grid[x][y][z] = 0;
                }
            }
        }

        // Reset the iteration count
        self.iteration_count = 0;

        // Reset the order parameter
        self.order_parameter = vec![];
    }



    // Reset is empty, just like the original gpu implemenetation (this is kind of a fallen signature for n-chemicals)
    fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) {
        
    }

    // Get, set and size are exactly the same as the original gpu implementation
    fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.grid[x][y][z] as u32
    }

    fn set(&mut self, x: usize, y: usize, z: usize, val: u32) {
        self.grid[x][y][z] = val as u8;
    }

    fn size(&self) -> usize {
        self.grid.len()
    }


    fn import_data_from_automaton(&mut self, other: &dyn CellularAutomaton3D) {

        // Loop over all (x,y,z) positions and copy the data
        for x in 0..self.size() {
            for y in 0..self.size() {
                for z in 0..self.size() {
                    self.set(x, y, z, other.get(x, y, z));
                }
            }
        }

        // Set the number of iterations identical to 'other'
        self.set_iteration_count(other.get_iteration_count());

        // Reset and recompute the order parameter
        self.order_parameter = vec![];

        self.compute_order_parameter();

    }


    // Spreading chemicals randomly is done in exactly the same way as the original implementation
    fn spread_chemicals_randomly(&mut self, chem: u32) {
        // Random number generator
        let mut rng = rand::thread_rng();

        // Loop over all the cells in the grid
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                for z in 0..self.grid.len() {
                    self.set(x, y, z, rng.gen_range(0..chem));
                }
            }
        }

        // Reset the iteration count
        self.iteration_count = 0;

        // Reset the order parameter
        self.order_parameter = vec![];

        self.compute_order_parameter();
    }





    //
    //
    // RUN ITERATION: This is where some parts had to be changed
    //
    //
    fn run_iteration(&mut self) {
        autoreleasepool(|| {

            let device = Device::system_default().expect("no device found");
            println!("Simulating on GPU: {}", device.name());
            let command_queue = device.new_command_queue();

            let data = self.export();
            let mut chemicals: Vec<f32> = vec![];

            // Load all the chemicals in this vector in the following order:
            // first range, then influence
            for c in &self.chemicals {
                chemicals.push(c.promote.influence);
                chemicals.push(c.demote.influence);
            }

            let buffer = device.new_buffer_with_data(
                unsafe { mem::transmute(data.as_ptr()) },
                (data.len() * mem::size_of::<u8>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            );

            let sum = {
                let data = self.export();
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<u8>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache,
                )
            };

            let arg_chemicals = {
                let data = chemicals.as_slice();
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<f32>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache
                )
            };

            // Computing relative neighbours: DC and UC
            let mut neighbours_promote: Vec<Vec<(i32, i32, i32)>> = vec![];
            let mut neighbours_demote: Vec<Vec<(i32, i32, i32)>> = vec![];

            // Make enough room for neighbours that are connected to each of the chemicals
            for c in &self.chemicals {
                neighbours_promote.push(vec![]);
                neighbours_demote.push(vec![]);
            }

            // Find the chemical with the largest range
            let mut max_range = self.chemicals[0].promote.range;

            for c in &self.chemicals {
                if c.promote.range > max_range {
                    max_range = c.promote.range;
                }

                if c.demote.range > max_range {
                    max_range = c.demote.range;
                }
            }

            let max_range_i = f32::ceil(max_range) as i32 + 2;

            for x in -max_range_i..max_range_i {
                for y in -max_range_i..max_range_i {
                    for z in -max_range_i..max_range_i {
                        // Compute the distance from the point (0, 0, 0)
                        let dist = f32::sqrt((x*x + y*y + z*z) as f32);

                        // A voxel cannot be its own neighbour, so (0, 0, 0) must be excluded from the neighbour-pack
                        if !(x == 0 && y == 0 && z == 0) {

                            // Loop over all the chemicals to determine whether it falls into its promotion
                            // or demotion range
                            for i in 0..self.chemicals.len() {

                                let c = &self.chemicals[i];

                                // If this point falls within the range of this promotion chemical
                                if dist <= c.promote.range {
                                    neighbours_promote[i].push((x, y, z));
                                }

                                // Else: if this point falls within the range of this demotion chemical
                                else if dist <= c.demote.range {
                                    neighbours_demote[i].push((x, y, z));
                                }

                            }
                        }
                        
                    }
                }
            }


            //
            // Construct buffers for the promoting and demoting chemical neighbours
            //
            let mut neighbours_promote_x: Vec<i32> = vec![];
            let mut neighbours_promote_y: Vec<i32> = vec![];
            let mut neighbours_promote_z: Vec<i32> = vec![];
            let mut neighbours_demote_x: Vec<i32> = vec![];
            let mut neighbours_demote_y: Vec<i32> = vec![];
            let mut neighbours_demote_z: Vec<i32> = vec![];

            for n_per_chem in &neighbours_promote {
                for n in n_per_chem {
                    neighbours_promote_x.push(n.0);
                    neighbours_promote_y.push(n.1);
                    neighbours_promote_z.push(n.2);
                }
            }

            for n_per_chem in &neighbours_demote {
                for n in n_per_chem {
                    neighbours_demote_x.push(n.0);
                    neighbours_demote_y.push(n.1);
                    neighbours_demote_z.push(n.2);
                }
            }

            let arg_neighbours_promote_x = device.new_buffer_with_data(
                unsafe { mem::transmute(neighbours_promote_x.as_slice().as_ptr()) },
                (neighbours_promote_x.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_neighbours_promote_y = device.new_buffer_with_data(
                unsafe { mem::transmute(neighbours_promote_y.as_slice().as_ptr()) },
                (neighbours_promote_y.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_neighbours_promote_z = device.new_buffer_with_data(
                unsafe { mem::transmute(neighbours_promote_z.as_slice().as_ptr()) },
                (neighbours_promote_z.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_neighbours_demote_x = device.new_buffer_with_data(
                unsafe { mem::transmute(neighbours_demote_x.as_slice().as_ptr()) },
                (neighbours_demote_x.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_neighbours_demote_y = device.new_buffer_with_data(
                unsafe { mem::transmute(neighbours_demote_y.as_slice().as_ptr()) },
                (neighbours_demote_y.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_neighbours_demote_z = device.new_buffer_with_data(
                unsafe { mem::transmute(neighbours_demote_z.as_slice().as_ptr()) },
                (neighbours_demote_z.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            // println!("Considering {} dc neighbours and {} uc neighbours", dc_neighbours_x.len(), uc_neighbours_x.len());

            // println!("The integral of influences over neighbours is {}", dc_neighbours_x.len() as f32 * self.dc_influence + uc_neighbours_x.len() as f32 * self.uc_influence);

            // There's a couple of size parameters we need to pass over to the gpu
            let mut size_container: Vec<u32> = vec![AUTOMATON_SIZE as u32, self.chemicals.len() as u32];

            // It contains the size of the automaton (1), the number of chemical-groups (2)
            // And for every chemical, the number of promoting and demoting neighbours
            for i in 0..self.chemicals.len() {
                size_container.push(neighbours_promote[i].len() as u32);
                size_container.push(neighbours_demote[i].len() as u32);
            }

            let arg_size_container = {
                let data = size_container.as_slice();
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<u32>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache
                )
            };

            let command_buffer = command_queue.new_command_buffer();
            let encoder = command_buffer.new_compute_command_encoder();

            let library = device
                .new_library_with_source(AUTOMATON_SHADER_SRC, &CompileOptions::new())
                .unwrap();
            let kernel = library.get_function("compute_iteration", None).unwrap();

            let argument_encoder = kernel.new_argument_encoder(0);
            let arg_buffer = device.new_buffer(
                argument_encoder.encoded_length(),
                MTLResourceOptions::empty(),
            );
            argument_encoder.set_argument_buffer(&arg_buffer, 0);
            argument_encoder.set_buffer(0, &buffer, 0);
            argument_encoder.set_buffer(1, &sum, 0);
            argument_encoder.set_buffer(2, &arg_size_container, 0);
            argument_encoder.set_buffer(3, &arg_chemicals, 0);
            argument_encoder.set_buffer(4, &arg_neighbours_promote_x, 0);
            argument_encoder.set_buffer(5, &arg_neighbours_promote_y, 0);
            argument_encoder.set_buffer(6, &arg_neighbours_promote_z, 0);
            argument_encoder.set_buffer(7, &arg_neighbours_demote_x, 0);
            argument_encoder.set_buffer(8, &arg_neighbours_demote_y, 0);
            argument_encoder.set_buffer(9, &arg_neighbours_demote_z, 0);

            let pipeline_state_descriptor = ComputePipelineDescriptor::new();
            pipeline_state_descriptor.set_compute_function(Some(&kernel));

            let pipeline_state = device
                .new_compute_pipeline_state_with_function(
                    pipeline_state_descriptor.compute_function().unwrap(),
                )
                .unwrap();

            encoder.set_compute_pipeline_state(&pipeline_state);
            encoder.set_buffer(0, Some(&arg_buffer), 0);

            encoder.use_resource(&buffer, MTLResourceUsage::Read);
            encoder.use_resource(&sum, MTLResourceUsage::Write);
            encoder.use_resource(&arg_size_container, MTLResourceUsage::Read);
            encoder.use_resource(&arg_chemicals, MTLResourceUsage::Read);
            encoder.use_resource(&arg_neighbours_promote_x, MTLResourceUsage::Read);
            encoder.use_resource(&arg_neighbours_promote_y, MTLResourceUsage::Read);
            encoder.use_resource(&arg_neighbours_promote_z, MTLResourceUsage::Read);
            encoder.use_resource(&arg_neighbours_demote_x, MTLResourceUsage::Read);
            encoder.use_resource(&arg_neighbours_demote_y, MTLResourceUsage::Read);
            encoder.use_resource(&arg_neighbours_demote_z, MTLResourceUsage::Read);

            
            
            let width = pipeline_state.thread_execution_width();
            let height = pipeline_state.max_total_threads_per_threadgroup() / width;

            let threads_per_grid = MTLSize {
                width: (data.len() as u64),
                height: 1,
                depth: 1,
            };

            let threads_per_thread_group = MTLSize {
                width,
                height,
                depth: 1,
            };

            encoder.dispatch_threads(threads_per_grid, threads_per_thread_group);
            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();

            let ptr = sum.contents() as *mut [u8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE];
            unsafe {
                self.import((*ptr).to_vec());
            }

        });

        self.iteration_count += 1;

        // Compute the new order parameter and insert it into the array
        self.compute_order_parameter();

    }





    // Getting and setting the iteration count is the same as the original gpu implementation
    fn set_iteration_count(&mut self, iterations: u32) {
        self.iteration_count = iterations;
    }

    fn get_iteration_count(&self) -> u32 {
        self.iteration_count
    }

    // Extracting Marching-cubes mesh is done in exactly the same way as the original gpu implementation
    fn mc_extract(&self, vertices: &mut Vec<f32>, indices: &mut Vec<u32>) {
        let mut mc = MarchingCubes::new(self.size());
        mc.extract(self, vertices, indices);
    }



}

impl Source for GPUNChemicalsCellularAutomaton3D {
    fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        // Assignment: return negative values for 'inside' and positive for 'outside'.
        // We'll return -1 for chemical 1 and +1 for chemical 0.

        // Caution: the source will be sampled between (0, 0, 0) and (1, 1, 1)

        let xindex = usize::min((x * (self.size() - 1) as f32).round() as usize, self.size() - 1);
        let yindex = usize::min((y * (self.size() - 1) as f32).round() as usize, self.size() - 1);
        let zindex = usize::min((z * (self.size() - 1) as f32).round() as usize, self.size() - 1);

        let chemical = self.get(xindex as usize, yindex as usize, zindex as usize);

        if chemical as usize == self.marching_cubes_chemical_capture {
            return 1.0;
        } else {
            return -1.0;
        }
    }
}