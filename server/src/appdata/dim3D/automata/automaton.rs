pub trait CellularAutomaton3D {
    fn clear_all_voxels(&mut self);
    fn reset(&mut self, size: usize, dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32);
    fn get(&self, x: usize, y: usize, z: usize) -> u32;
    fn set(&mut self, x: usize, y: usize, z: usize, val: u32);
    fn size(&self) -> usize;
    fn spread_chemicals_randomly(&mut self, chem: u32);
    fn run_iteration(&mut self);
    fn get_iteration_count(&self) -> u32;
    fn compare(&self, other: &dyn CellularAutomaton3D) -> bool {
        // If the automata are of different size, they can't be equal
        if self.size() != other.size() {
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
}