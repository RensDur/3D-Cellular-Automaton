use isosurface::{source::Source, marching_cubes::MarchingCubes};

use super::automaton::CellularAutomaton3D;




pub struct CellularAutomaton3DChunk {
    pub grid: Vec<Vec<Vec<u8>>>
}

impl CellularAutomaton3DChunk {
    pub fn new(size: usize) -> Self {
        CellularAutomaton3DChunk { grid: vec![vec![vec![0; size]; size]; size] }
    }
}

impl CellularAutomaton3D for CellularAutomaton3DChunk {
    fn clear_all_voxels(&mut self) {
        for x in 0..self.size() {
            for y in 0..self.size() {
                for z in 0..self.size() {
                    self.grid[x][y][z] = 0;
                }
            }
        }
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
        
    }

    fn run_iteration(&mut self) {
        
    }

    fn set_iteration_count(&mut self, iterations: u32) {
        
    }

    fn get_iteration_count(&self) -> u32 {
        0
    }

    fn mc_extract(&self, vertices: &mut Vec<f32>, indices: &mut Vec<u32>) {
        let mut mc = MarchingCubes::new(self.size());
        mc.extract(self, vertices, indices);
    }
}


impl Source for CellularAutomaton3DChunk {
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