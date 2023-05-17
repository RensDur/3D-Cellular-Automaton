use marching_cubes::marching::{MarchingCubes, GridCell, Triangle};
use serde::Serialize;
use crate::appdata::dim3d::automata::automaton::CellularAutomaton3D;




/**
 * Struct: a serialisable version of the triangle that's provided by the Marching Cubes library
 */
#[derive(Serialize)]
pub struct MeshTriangle {
    pub vertices: [[f32; 3]; 3]
}


/**
 * Struct: create a contract between the Cellular Automaton implementation and the Marching Cubes library
 */
struct MCVoxel {
    positions: Vec<[f32; 3]>,
    values: Vec<f32>
}

impl MCVoxel {

    pub fn new() -> Self {
        Self {
            positions: vec![],
            values: vec![]
        }
    }

    pub fn add(&mut self, x: usize, y: usize, z: usize, value: u32) {
        self.positions.push([x as f32, y as f32, z as f32]);
        self.values.push(value as f32);
    }

    pub fn get_positions(&self) -> [[f32; 3]; 8] {
        let mut output = [[0f32; 3]; 8];

        for p in 0..8 {
            output[p] = self.positions[p];
        }

        output
    }

    pub fn get_values(&self) -> [f32; 8] {
        let mut output = [0f32; 8];

        for p in 0..8 {
            output[p] = self.values[p];
        }

        output
    }

}


pub fn generate_triangles_marching_cubes(ca: &dyn CellularAutomaton3D) -> Vec<MeshTriangle> {

    // Create a vector that stores all the triangles that form the surface between the two chemicals.
    let mut all_triangles: Vec<MeshTriangle> = vec![];

    // The isolevel determines the value at which separation of the chemicals occurs
    let isolevel: f32 = 0.5;

    // Loop over all voxels in the cellular automaton
    for x in 0..(ca.size() - 1) {
        for y in 0..(ca.size() - 1) {
            for z in 0..(ca.size() - 1) {

                // Create an instance of MCVoxel (defined above) to store all information about this voxel
                let mut mc_voxel = MCVoxel::new();

                // Add all eight points of this voxel
                for dx in 0..2 {
                    for dy in 0..2 {
                        for dz in 0..2 {
                            mc_voxel.add(x+dx, y+dy, z+dz, ca.get(x+dx, y+dy, z+dz));
                        }
                    }
                }

                // Transform this into an instance of the GridCell struct provided by the library
                let grid = GridCell {
                    positions: mc_voxel.get_positions(),
                    value: mc_voxel.get_values()
                };

                // Create a container that stores the triangles inside this voxel
                let mut triangles: Vec<Triangle> = vec![];

                // Create an instance of MarchingCubes
                let mc = MarchingCubes::new(isolevel, grid);

                // Generate the triangles and add them to the all_triangles vector
                mc.polygonise(&mut triangles);

                for t in triangles {
                    all_triangles.push(MeshTriangle { vertices: t.positions });
                }
            }
        }
    }
    
    all_triangles
}