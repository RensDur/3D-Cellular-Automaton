use crate::gltfgeneration::gltf_generation::generate_large_gltf;
use crate::gltfgeneration::garland_heckbert::GarlandHeckbert;
use super::automaton_cpu::MeshTriangle;

pub trait CellularAutomaton3D {
    fn clear_all_voxels(&mut self);
    fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32);
    fn get(&self, x: usize, y: usize, z: usize) -> u32;
    fn set(&mut self, x: usize, y: usize, z: usize, val: u32);
    fn size(&self) -> usize;
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

    }
    fn spread_chemicals_randomly(&mut self, chem: u32);
    fn run_iteration(&mut self);
    fn set_iteration_count(&mut self, iterations: u32);
    fn get_iteration_count(&self) -> u32;
    fn compare(&self, other: &dyn CellularAutomaton3D) -> bool {
        // If the automata are of different size, they can't be equal
        if self.size() != other.size() {
            return false;
        }

        // If the automata are at a different iteration-count, they can't be equal
        if self.get_iteration_count() != other.get_iteration_count() {
            return false;
        }

        // Otherwise, continue and check the values of every cell
        for x in 0..self.size() {
            for y in 0..self.size() {
                for z in 0..self.size() {
                    if self.get(x, y, z) != other.get(x, y, z) {
                        return false;
                    }
                }
            }
        }

        // No mismatches have been found, return true
        true
    }

    // Methods concerned with Marching Cubes
    fn mc_extract(&self, vertices: &mut Vec<f32>, indices: &mut Vec<u32>);
    fn get_marching_cubes_mesh(&self) -> String {
        // Create a vector that stores all the triangles that form the surface between the two chemicals.
        let mut all_triangles: Vec<[f32; 3]> = vec![];

        // Loop over all voxels in the cellular automaton
        let mut vertices: Vec<f32> = vec![];
        let mut indices: Vec<u32> = vec![];

        // This is done by utilising the mc_extract method
        // that's part of this contract.
        self.mc_extract(&mut vertices, &mut indices);

        GarlandHeckbert::simplify(&vertices, &indices, 0.1);

        println!("Vertices and indices extracted:\n\tVertices: {}\n\tIndices: {}", vertices.len(), indices.len());

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
            all_triangles.push(vertices_coords[indices[i] as usize]);
            all_triangles.push(vertices_coords[indices[i+1] as usize]);
            all_triangles.push(vertices_coords[indices[i+2] as usize]);
            //     MeshTriangle {
            //         vertices: [
            //             vertices_coords[indices[i] as usize],
            //             vertices_coords[indices[i+1] as usize],
            //             vertices_coords[indices[i+2] as usize]
            //         ]
            //     }
            // );
        }

        if all_triangles.len() > 0 {
            return generate_large_gltf(&all_triangles.as_slice()).unwrap();
        }

        String::from("{}")
    }
}