use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CAGrid3D {
    size: usize,
    pub data: Vec<Vec<Vec<u32>>>
}

impl CAGrid3D {
    pub fn new(size: usize) -> CAGrid3D {
        let data: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; size]; size]; size];
        CAGrid3D {
            size,
            data
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u32 {
        self.data[x][y][z]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u32) {
        self.data[x][y][z] = val;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dist(x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2: usize) -> f32 {
        let dx = x2 as f32 - x1 as f32;
        let dy = y2 as f32 - y1 as f32;
        let dz = z2 as f32 - z1 as f32;
    
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}
