use std::sync::Mutex;

use actix_web::{post, web, Responder, Result};
use crate::{CAAppData, appdata::dim3d::automata::{automaton::CellularAutomaton3D, automaton_cpu::CPUCellularAutomaton3D, automaton_gpu::GPUCellularAutomaton3D}, AUTOMATON_SIZE};

#[post("/benchmarks/gpu-shader-increment")]
async fn benchmarks_gpu_shader_increment(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let mut state_mod = state.lock().unwrap();

    // For this method to work, it is required that the shader simply updates each voxel with the gid it was computed with

    // Run 1 iteration on the gpu
    state_mod.gpu_ca.run_iteration();

    // Check whether each cell follows the formula (x + size*y + size*size*z)
    let mut result = true;

    for x in 0..AUTOMATON_SIZE {
        for y in 0..AUTOMATON_SIZE {
            for z in 0..AUTOMATON_SIZE {
                if state_mod.gpu_ca.get(x, y, z) != (x + y*AUTOMATON_SIZE + z*AUTOMATON_SIZE*AUTOMATON_SIZE) as u32 {
                    result = false;
                }
            }
        }
    }

    drop(state_mod);

    let message: &str;

    if result {
        message = "Complete match";
    } else {
        message = "Outcome mismatch";
    }

    Ok(message)
}