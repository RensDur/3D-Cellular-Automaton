use super::{automaton::CellularAutomaton3D, automaton_cpu::MeshTriangle};

use isosurface::{source::Source, marching_cubes::MarchingCubes};
use serde::{Serialize, Deserialize};

use rand::prelude::*;

use metal::*;
use objc::rc::autoreleasepool;
use std::mem;

use crate::{AUTOMATON_SIZE, routes::gpu_get};

const AUTOMATON_SHADER_SRC: &str = include_str!("automaton_shader.metal");

#[derive(Clone, Serialize, Deserialize)]
pub struct GPUCellularAutomaton3D {
    pub grid: Vec<Vec<Vec<u8>>>,
    pub dc_range: f32,
    pub dc_influence: f32,
    pub uc_range: f32,
    pub uc_influence: f32,
    iteration_count: u32
}

impl GPUCellularAutomaton3D {

    pub fn new(dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) -> Self {
        GPUCellularAutomaton3D {
            grid: vec![vec![vec![0u8; AUTOMATON_SIZE]; AUTOMATON_SIZE]; AUTOMATON_SIZE],
            dc_range,
            dc_influence,
            uc_range,
            uc_influence,
            iteration_count: 0
        }
    }

    /**
     * Method: transform this automaton into a mesh using Marching Cubes
     */
    pub fn get_marching_cubes_mesh(&self) -> Vec<MeshTriangle> {

        // Create a vector that stores all the triangles that form the surface between the two chemicals.
        let mut all_triangles: Vec<MeshTriangle> = vec![];

        // Loop over all voxels in the cellular automaton
        let mut mc = MarchingCubes::new(self.size());

        let mut vertices: Vec<f32> = vec![];
        let mut indices: Vec<u32> = vec![];

        mc.extract(self, &mut vertices, &mut indices);

        // 'vertices' contains (x, y, z) values in sequential manner
        // 'indices' creates triangles by indexing three vertices sequentially
        
        // 1. Transform 'vertices' into (x,y,z) coordinates (group by 3)
        if vertices.len() % 3 != 0 {
            panic!("Marching Cubes: vertices array length not multiple of three");
        }

        let mut vertices_coords: Vec<[f32; 3]> = vec![];

        for v in (0..vertices.len()).step_by(3) {
            // v:   x
            // v+1: y
            // v+2: z
            vertices_coords.push([vertices[v] * self.size() as f32, vertices[v+1] * self.size() as f32, vertices[v+2] * self.size() as f32]);
        }

        // 2. Transform 'indices' into triangles (group by three vertices)
        if indices.len() % 3 != 0 {
            panic!("Marching Cubes: indices array length not multiple of three");
        }

        for i in (0..indices.len()).step_by(3) {
            // i:   vertex 1
            // i+1: vertex 2
            // i+2: vertex 3
            all_triangles.push(
                MeshTriangle {
                    vertices: [
                        vertices_coords[indices[i] as usize],
                        vertices_coords[indices[i+1] as usize],
                        vertices_coords[indices[i+2] as usize]
                    ]
                }
            );
        }

        all_triangles

    }

    // fn export(&self) -> [u8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE] {

    //     let mut res = [0u8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE];

    //     for x in 0..AUTOMATON_SIZE {
    //         for y in 0..AUTOMATON_SIZE {
    //             for z in 0..AUTOMATON_SIZE {
    //                 res[x + y*AUTOMATON_SIZE + z*AUTOMATON_SIZE*AUTOMATON_SIZE] = self.grid[x][y][z];
    //             }
    //         }
    //     }

    //     res

    // }

    // fn import(&mut self, data: [u8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE]) {

    //     for x in 0..AUTOMATON_SIZE {
    //         for y in 0..AUTOMATON_SIZE {
    //             for z in 0..AUTOMATON_SIZE {
    //                 self.grid[x][y][z] = data[x + y*AUTOMATON_SIZE + z*AUTOMATON_SIZE*AUTOMATON_SIZE];
    //             }
    //         }
    //     }

    // }

    fn export(&self) -> Vec<u8> {

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

        // Reset the iteration count
        self.iteration_count = 0;
    }

    fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) {
        
    }

    fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.grid[x][y][z] as u32
    }

    fn set(&mut self, x: usize, y: usize, z: usize, val: u32) {
        self.grid[x][y][z] = val as u8;
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

        // Reset the iteration count
        self.iteration_count = 0;
    }

    fn run_iteration(&mut self) {
        autoreleasepool(|| {

            let device = Device::system_default().expect("no device found");
            println!("Simulating on GPU: {}", device.name());
            let command_queue = device.new_command_queue();

            let data = self.export();
            let chemicals = [self.dc_range, self.dc_influence, self.uc_range, self.uc_influence];

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
                let data = chemicals;
                device.new_buffer_with_data(
                    unsafe { mem::transmute(data.as_ptr()) },
                    (data.len() * mem::size_of::<f32>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache
                )
            };

            // Computing relative neighbours: DC and UC
            let mut dc_neighbours_x: Vec<i32> = vec![];
            let mut dc_neighbours_y: Vec<i32> = vec![];
            let mut dc_neighbours_z: Vec<i32> = vec![];
            let mut uc_neighbours_x: Vec<i32> = vec![];
            let mut uc_neighbours_y: Vec<i32> = vec![];
            let mut uc_neighbours_z: Vec<i32> = vec![];

            // UC has a larger range than DC, so pull it up to the closest larger integer and use it as range
            let uc_range = f32::ceil(self.uc_range) as i32 + 2; // +1 as the x..y excludes y

            for x in -uc_range..uc_range {
                for y in -uc_range..uc_range {
                    for z in -uc_range..uc_range {
                        // Compute the distance from the point (0, 0, 0)
                        let dist = f32::sqrt((x*x + y*y + z*z) as f32);

                        // A voxel cannot be its own neighbour, so (0, 0, 0) must be excluded from the neighbour-pack
                        if !(x == 0 && y == 0 && z == 0) {
                            // If this point falls within the range of Differentiated Cells
                            if dist <= self.dc_range {
                                // Append it to the dc_neighbour pack
                                dc_neighbours_x.push(x);
                                dc_neighbours_y.push(y);
                                dc_neighbours_z.push(z);
                            }

                            // Else: if this point falls within the range of Undifferentiated Cells
                            else if dist <= self.uc_range {
                                // Append it to the uc_neighbour pack
                                uc_neighbours_x.push(x);
                                uc_neighbours_y.push(y);
                                uc_neighbours_z.push(z);
                            }
                        }
                        
                    }
                }
            }

            let arg_dc_neighbours_x = device.new_buffer_with_data(
                unsafe { mem::transmute(dc_neighbours_x.as_slice().as_ptr()) },
                (dc_neighbours_x.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_dc_neighbours_y = device.new_buffer_with_data(
                unsafe { mem::transmute(dc_neighbours_y.as_slice().as_ptr()) },
                (dc_neighbours_y.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_dc_neighbours_z = device.new_buffer_with_data(
                unsafe { mem::transmute(dc_neighbours_z.as_slice().as_ptr()) },
                (dc_neighbours_z.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_uc_neighbours_x = device.new_buffer_with_data(
                unsafe { mem::transmute(uc_neighbours_x.as_ptr()) },
                (uc_neighbours_x.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_uc_neighbours_y = device.new_buffer_with_data(
                unsafe { mem::transmute(uc_neighbours_y.as_ptr()) },
                (uc_neighbours_y.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            let arg_uc_neighbours_z = device.new_buffer_with_data(
                unsafe { mem::transmute(uc_neighbours_z.as_ptr()) },
                (uc_neighbours_z.len() * mem::size_of::<i32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
            );

            println!("Considering {} dc neighbours and {} uc neighbours", dc_neighbours_x.len(), uc_neighbours_x.len());

            println!("The integral of influences over neighbours is {}", dc_neighbours_x.len() as f32 * self.dc_influence + uc_neighbours_x.len() as f32 * self.uc_influence);

            let arg_size_container = {
                let data: [u32; 3] = [AUTOMATON_SIZE as u32, dc_neighbours_x.len() as u32, uc_neighbours_x.len() as u32];
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
            argument_encoder.set_buffer(4, &arg_dc_neighbours_x, 0);
            argument_encoder.set_buffer(5, &arg_dc_neighbours_y, 0);
            argument_encoder.set_buffer(6, &arg_dc_neighbours_z, 0);
            argument_encoder.set_buffer(7, &arg_uc_neighbours_x, 0);
            argument_encoder.set_buffer(8, &arg_uc_neighbours_y, 0);
            argument_encoder.set_buffer(9, &arg_uc_neighbours_z, 0);

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
            encoder.use_resource(&arg_dc_neighbours_x, MTLResourceUsage::Read);
            encoder.use_resource(&arg_dc_neighbours_y, MTLResourceUsage::Read);
            encoder.use_resource(&arg_dc_neighbours_z, MTLResourceUsage::Read);
            encoder.use_resource(&arg_uc_neighbours_x, MTLResourceUsage::Read);
            encoder.use_resource(&arg_uc_neighbours_y, MTLResourceUsage::Read);
            encoder.use_resource(&arg_uc_neighbours_z, MTLResourceUsage::Read);

            
            
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

            let ptr = sum.contents() as *mut [u8; AUTOMATON_SIZE*AUTOMATON_SIZE*AUTOMATON_SIZE];
            unsafe {
                self.import((*ptr).to_vec());
            }

        });

        self.iteration_count += 1;

    }

    fn set_iteration_count(&mut self, iterations: u32) {
        self.iteration_count = iterations;
    }

    fn get_iteration_count(&self) -> u32 {
        self.iteration_count
    }

}


impl Source for GPUCellularAutomaton3D {
    fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        // Assignment: return negative values for 'inside' and positive for 'outside'.
        // We'll return -1 for chemical 1 and +1 for chemical 0.

        // Caution: the source will be sampled between (0, 0, 0) and (1, 1, 1)

        let xindex = usize::min((x * (self.size() - 1) as f32).round() as usize, self.size() - 1);
        let yindex = usize::min((y * (self.size() - 1) as f32).round() as usize, self.size() - 1);
        let zindex = usize::min((z * (self.size() - 1) as f32).round() as usize, self.size() - 1);

        let chemical = self.get(xindex as usize, yindex as usize, zindex as usize);

        if chemical == 0 {
            return 1.0;
        } else {
            return -1.0;
        }
    }
}