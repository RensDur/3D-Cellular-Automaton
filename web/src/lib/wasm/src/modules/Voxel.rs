mod voxels {
    /**
     * Voxel trait that abstracts the concept of a voxel
     */
    pub trait Voxel<T> {
        fn get(&self) -> T;
        fn set(&mut self, val: &T);
        fn equals(&self, other: &Voxel<T>) -> boo;
    }


    pub struct CAVoxel {
        id: u32
    }

    impl Voxel<u32> for CAVoxel {
        fn get(&self) -> u32 {
            self.id
        }

        fn set(&mut self, val: u32) {
            self.id = val;
        }

        fn equals(&self, other: &Voxel<u32>) -> bool {
            self.id == other.id;
        }
    }

}