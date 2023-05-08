use super::automaton::CellularAutomaton3D;

use serde::{Serialize, Deserialize};

use rand::prelude::*;

use metal::*;
use objc::rc::autoreleasepool;
use std::mem;

const AUTOMATON_SHADER_SRC: &str = include_str!("automaton_shader.metal");
pub const AUTOMATON_SIZE: usize = 20;
const CHEMICALS: [f32; 4] = [2.0, 1.0, 4.0, -0.25];

#[derive(Clone, Serialize, Deserialize)]
pub struct GPUCellularAutomaton3D {
    pub grid: [[[u32; AUTOMATON_SIZE]; AUTOMATON_SIZE]; AUTOMATON_SIZE],
    pub dc_range: f32,
    pub dc_influence: f32,
    pub uc_range: f32,
    pub uc_influence: f32
}

impl GPUCellularAutomaton3D {


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

            // 1. Find a GPU device
            let device = Device::system_default().expect("Default GPU device not found");

            // 2. Get a reference to the Metal function
            // 2.1. Ask the MTLDevice to create a MTLLibrary object for the default library
            let library = device
                .new_library_with_source(AUTOMATON_SHADER_SRC, &CompileOptions::new())
                .unwrap();

            // 2.2. Ask the MTLLibrary for a MTLFunction object that represents our Metal shader
            let kernel = library.get_function("compute_iteration", None).unwrap();

            // 3. Prepare a Metal Compute Pipeline
            let pipeline_state_descriptor = ComputePipelineDescriptor::new();
            pipeline_state_descriptor.set_compute_function(Some(&kernel));

            let pipeline_state = device
                .new_compute_pipeline_state_with_function(
                    pipeline_state_descriptor.compute_function().unwrap(),
                )
                .unwrap();

            // 4. Create a Command Queue
            //    To send work to the GPU, you need a command queue. Metal uses command queues to schedule commands.
            let command_queue = device.new_command_queue();

            // 5. Create Data Buffers and Load Data
            let data_args = CHEMICALS;
            let data_input = self.grid.clone();
            let data_output = [[[0u32; AUTOMATON_SIZE]; AUTOMATON_SIZE]; AUTOMATON_SIZE];

            // The args buffer holds the values for respectively rc_range, rc_influence, uc_range and uc_influence
            let data_args_buffer = device.new_buffer_with_data(
                unsafe { mem::transmute(data_args.as_ptr()) },
                (data_args.len() * mem::size_of::<f32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            // The input buffer holds the current values for each pigment cell
            // let data_input_buffer = device.new_buffer_with_data(
            //     unsafe { mem::transmute(data_input.as_ptr()) },
            //     (data_input.len() * mem::size_of::<[[u32; AUTOMATON_SIZE]; AUTOMATON_SIZE]>()) as u64,
            //     MTLResourceOptions::CPUCacheModeDefaultCache
            // );

            // Create a texture-descriptor of the type R32Uint with 3D automaton size
            let data_input_texture_descriptor = TextureDescriptor::new();
            data_input_texture_descriptor.set_pixel_format(MTLPixelFormat::R32Uint);
            data_input_texture_descriptor.set_width(AUTOMATON_SIZE as u64);
            data_input_texture_descriptor.set_height(AUTOMATON_SIZE as u64);
            data_input_texture_descriptor.set_depth(AUTOMATON_SIZE as u64);

            // Create a texture for the data input corresponding to this descriptor for the
            // installed device.
            let data_input_texture = device.new_texture(&data_input_texture_descriptor);
            
            // Define the region in which this texture will act
            let data_input_region = MTLRegion {
                origin: MTLOrigin {x: 0, y: 0, z: 0},
                size: MTLSize {
                    width: AUTOMATON_SIZE as u64,
                    height: AUTOMATON_SIZE as u64,
                    depth: AUTOMATON_SIZE as u64
                }
            };

            // Fill the texture with the current CA data
            data_input_texture.replace_region(
                data_input_region,
                0,
                unsafe { mem::transmute(data_input.as_ptr()) },
                4*AUTOMATON_SIZE as u64
            );

            // The output buffer will hold the new values for each pigment cell
            // let data_output_buffer = device.new_buffer_with_data(
            //     unsafe { mem::transmute(data_output.as_ptr()) },
            //     (data_output.len() * mem::size_of::<[[u32; AUTOMATON_SIZE]; AUTOMATON_SIZE]>()) as u64,
            //     MTLResourceOptions::CPUCacheModeDefaultCache
            // );

            // Create a texture-descriptor of the type R32Uint with 3D automaton size
            let data_output_texture_descriptor = TextureDescriptor::new();
            data_output_texture_descriptor.set_pixel_format(MTLPixelFormat::R32Uint);
            data_output_texture_descriptor.set_width(AUTOMATON_SIZE as u64);
            data_output_texture_descriptor.set_height(AUTOMATON_SIZE as u64);
            data_output_texture_descriptor.set_depth(AUTOMATON_SIZE as u64);

            // Create a texture for the data output corresponding to this descriptor for the
            // installed device.
            // This is the texture where the resuting CA generation will be simulated through the shaders
            let data_output_texture = device.new_texture(&data_output_texture_descriptor);

            // 6. Create a Command Buffer
            let command_buffer = command_queue.new_command_buffer();

            // 7. Create a Command Encoder
            let encoder = command_buffer.new_compute_command_encoder();

            // 8. Set Pipeline State and Argument Data
            // 8.1. Set the pipeline state object of the pipeline you want the command to execute
            encoder.set_compute_pipeline_state(&pipeline_state);

            // 8.2. Set data for any arguments that the pipeline needs to send into the shader function.
            encoder.set_buffer(0, Some(&data_args_buffer), 0);
            encoder.set_texture(1, Some(&data_input_texture));
            encoder.set_texture(2, Some(&data_output_texture));

            // encoder.use_resource(&data_input_buffer, MTLResourceUsage::Read);
            // encoder.use_resource(&data_output_buffer, MTLResourceUsage::Write);

            // 9. Specify Thread Count and Organisation
            //    Decide how many threads to create and how to organize those threads. Metal can create 3D grids.
            let grid_size = MTLSize {
                width: AUTOMATON_SIZE as u64,
                height: AUTOMATON_SIZE as u64,
                depth: AUTOMATON_SIZE as u64,
            };

            //    Metal subdivides the grid into smaller grids called threadgroups.
            //    Each threadgroup is calculated separately
            let thread_group_width = pipeline_state.thread_execution_width();
            let thread_group_height = pipeline_state.thread_execution_width();
            let mut threads_per_thread_group = pipeline_state.max_total_threads_per_threadgroup() / thread_group_width / thread_group_height;
            
            // If the selected thread-group-size is larger than the size of the automaton, use the automaton-size as group-size
            if threads_per_thread_group > AUTOMATON_SIZE as u64 {
                threads_per_thread_group = AUTOMATON_SIZE as u64;
            }

            let thread_group_size = MTLSize {
                width: threads_per_thread_group,
                height: threads_per_thread_group,
                depth: threads_per_thread_group
            };

            // 10. Encode the Compute Command to Execute the Threads
            encoder.dispatch_thread_groups(grid_size, thread_group_size);

            // 11. End the Compute Pass
            encoder.end_encoding();

            // 12. Commit the Command Buffer to Execute its Commands
            //     At this point, Metal will asynchronously commit the commands into the queue
            command_buffer.commit();

            // 13. Wait for the Calculation to Complete
            command_buffer.wait_until_completed();

            // 14. Extract the information from the GPU and update the information in this Automaton
            data_output_texture.get_bytes(
                unsafe { mem::transmute(data_input.as_ptr()) },
                4*AUTOMATON_SIZE as u64,
                data_input_region,
                0
            );

            self.grid = data_input;


        });
    }

}