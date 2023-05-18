

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