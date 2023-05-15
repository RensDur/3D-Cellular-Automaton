use std::sync::Mutex;

use actix_web::{post, web, Responder, Result};
use crate::{CAAppData, appdata::dim3d::automata::{automaton::CellularAutomaton3D, automaton_cpu::CPUCellularAutomaton3D, automaton_gpu::GPUCellularAutomaton3D}};


/**
 * Helper function: run correctness benchmark and produce human-readable feedback
 */
fn compare_with_human_feedback(cpu_ca: &CPUCellularAutomaton3D, gpu_ca: &GPUCellularAutomaton3D) -> String {
    
    let message: String;

    // If cpu and gpu at a different number of iterations, make notice
    if cpu_ca.get_iteration_count() != gpu_ca.get_iteration_count() {

        message = format!("Generation mismatch: CPU[{}] versus GPU[{}]\nYou can request to run additional iterations until both simulations live in the same generation and compare them automatically.", cpu_ca.get_iteration_count(), gpu_ca.get_iteration_count());
   
    } else {

        let comparison = cpu_ca.compare(gpu_ca);

        if comparison {
            message = format!("Complete match");
        } else {
            message = format!("Outcome mismatch");
        }

    }

    message
}


#[post("/benchmarks/compare-cpu-gpu")]
async fn benchmarks_compare_cpu_gpu(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let state_mod = state.lock().unwrap();

    let message = compare_with_human_feedback(&state_mod.cpu_ca, &state_mod.gpu_ca);

    drop(state_mod);

    Ok(message)
}

#[post("/benchmarks/compare-cpu-gpu-catch-up")]
async fn benchmarks_compare_cpu_gpu_catch_up(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let mut state_mod = state.lock().unwrap();

    // While the cpu has run less iterations than the gpu, let the cpu run additional iterations
    while state_mod.cpu_ca.get_iteration_count() < state_mod.gpu_ca.get_iteration_count() {
        state_mod.cpu_ca.run_iteration();
    }

    // While the gpu has run less iterations than the cpu, let the gpu run additional iterations
    while state_mod.gpu_ca.get_iteration_count() < state_mod.cpu_ca.get_iteration_count() {
        state_mod.gpu_ca.run_iteration();
    }

    let message = compare_with_human_feedback(&state_mod.cpu_ca, &state_mod.gpu_ca);

    drop(state_mod);

    Ok(message)
}