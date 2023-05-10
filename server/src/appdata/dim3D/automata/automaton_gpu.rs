use super::automaton::CellularAutomaton3D;

use serde::{Serialize, Deserialize};

use rand::prelude::*;

use metal::*;
use objc::rc::autoreleasepool;
use std::mem;

const AUTOMATON_SHADER_SRC: &str = include_str!("automaton_shader.metal");
pub const AUTOMATON_SIZE: usize = 50;
const CHEMICALS: [f32; 4] = [2.3, 1.0, 4.4, -0.19];

#[derive(Clone, Serialize, Deserialize)]
pub struct GPUCellularAutomaton3D {
    pub grid: Vec<Vec<Vec<u32>>>,
    pub dc_range: f32,
    pub dc_influence: f32,
    pub uc_range: f32,
    pub uc_influence: f32
}

impl GPUCellularAutomaton3D {

    pub fn new(size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) -> Self {
        GPUCellularAutomaton3D {
            grid: vec![vec![vec![0u32; AUTOMATON_SIZE]; AUTOMATON_SIZE]; AUTOMATON_SIZE],
            dc_range,
            dc_influence,
            uc_range,
            uc_influence
        }
    }

    fn export(&self) -> [u32; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE] {

        let mut res = [0u32; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE];

        for x in 0..AUTOMATON_SIZE {
            for y in 0..AUTOMATON_SIZE {
                for z in 0..AUTOMATON_SIZE {
                    res[x + y*AUTOMATON_SIZE + z*AUTOMATON_SIZE*AUTOMATON_SIZE] = self.grid[x][y][z];
                }
            }
        }

        res

    }

    fn import(&mut self, data: [u32; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE]) {

        for x in 0..AUTOMATON_SIZE {
            for y in 0..AUTOMATON_SIZE {
                for z in 0..AUTOMATON_SIZE {
                    self.grid[x][y][z] = data[x + y*AUTOMATON_SIZE + z*AUTOMATON_SIZE*AUTOMATON_SIZE];
                }
            }
        }

    }

}


impl CellularAutomaton3D for GPUCellularAutomaton3D {

    fn clear_all_voxels(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                for z in 0..self.grid.len() {
                    self.grid[x][y][z] = 0;
                }
            }
        }
    }

    fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) {
        
    }

    fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.grid[x][y][z]
    }

    fn set(&mut self, x: usize, y: usize, z: usize, val: u32) {
        self.grid[x][y][z] = val;
    }

    fn size(&self) -> usize {
        self.grid.len()
    }

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
    }

    fn run_iteration(&mut self) {
        autoreleasepool(|| {

            let device = Device::system_default().expect("no device found");
            println!("Simulating on GPU: {}", device.name());
            let command_queue = device.new_command_queue();

            let data = self.export();

            let buffer = device.new_buffer_with_data(
                unsafe { mem::transmute(data.as_ptr()) },
                (data.len() * mem::size_of::<u32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            );

            let sum = {
                let data = self.export();
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<u32>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache,
                )
            };

            let arg_chemicals = {
                let data = CHEMICALS;
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<f32>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache
                )
            };

            let dc_neighbours = {
                let mut data: Vec<i32> = vec![];

                let dc_range = f32::ceil(CHEMICALS[0]) as i32 + 10;

                for x in -dc_range..dc_range+1 {
                    for y in -dc_range..dc_range+1 {
                        for z in -dc_range..dc_range+1 {
                            // Comparing to (0, 0, 0)
                            let dist = f32::sqrt((x*x + y*y + z*z) as f32);

                            if dist <= CHEMICALS[0] && !(x == 0 && y == 0 && z == 0) {
                                data.push(x + y*AUTOMATON_SIZE as i32 + z*AUTOMATON_SIZE as i32*AUTOMATON_SIZE as i32);
                            }
                        }
                    }
                }

                data
            };

            let arg_dc_neighbours = device.new_buffer_with_data(
                unsafe { mem::transmute(dc_neighbours.as_ptr()) },
                (dc_neighbours.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );


            let uc_neighbours = {
                let mut data: Vec<i32> = vec![];

                let uc_range = f32::ceil(CHEMICALS[2]) as i32 + 10;

                for x in -uc_range..uc_range+1 {
                    for y in -uc_range..uc_range+1 {
                        for z in -uc_range..uc_range+1 {
                            // Comparing to (0, 0, 0)
                            let dist = f32::sqrt((x*x + y*y + z*z) as f32);

                            if dist <= CHEMICALS[2] && dist > CHEMICALS[0] && !(x == 0 && y == 0 && z == 0) {
                                data.push(x + y*AUTOMATON_SIZE as i32 + z*AUTOMATON_SIZE as i32*AUTOMATON_SIZE as i32);
                            }
                        }
                    }
                }

                data
            };

            println!("Considering {} dc neighbours and {} uc neighbours", dc_neighbours.len(), uc_neighbours.len());

            let arg_uc_neighbours = device.new_buffer_with_data(
                unsafe { mem::transmute(uc_neighbours.as_ptr()) },
                (uc_neighbours.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_size_container = {
                let data: [u32; 3] = [AUTOMATON_SIZE as u32, dc_neighbours.len() as u32, uc_neighbours.len() as u32];
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
            argument_encoder.set_buffer(4, &arg_dc_neighbours, 0);
            argument_encoder.set_buffer(5, &arg_uc_neighbours, 0);

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
            encoder.use_resource(&arg_dc_neighbours, MTLResourceUsage::Read);
            encoder.use_resource(&arg_uc_neighbours, MTLResourceUsage::Read);

            
            
            let width = 50*50;

            let thread_group_count = MTLSize {
                width,
                height: 1,
                depth: 1,
            };

            let thread_group_size = MTLSize {
                width: (data.len() as u64 + width) / width,
                height: 1,
                depth: 1,
            };

            encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();

            let ptr = sum.contents() as *mut [u32; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE];
            unsafe {
                self.import(*ptr);
            }

        });
    }

}