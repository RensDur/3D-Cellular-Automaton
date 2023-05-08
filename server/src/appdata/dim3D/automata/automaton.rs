pub trait CellularAutomaton3D {
    fn clear_all_voxels(&mut self);
    fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32);
    fn get(&self, x: usize, y: usize, z: usize) -> u32;
    fn set(&mut self, x: usize, y: usize, z: usize, val: u32);
    fn size(&self) -> usize;
    fn spread_chemicals_randomly(&mut self, chem: u32);
    fn run_iteration(&mut self);
}