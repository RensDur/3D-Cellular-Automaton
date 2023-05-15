use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::appdata::dim3d::automata::automaton::CellularAutomaton3D;
use crate::appdata::dim3d::automata::automaton_cpu::CPUCellularAutomaton3D;
use crate::appdata::dim3d::automata::automaton_gpu::GPUCellularAutomaton3D;
use crate::CAAppData;



#[get("/gpu/get-current-state")]
async fn gpu_get_current_state(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let response = web::Json(state_mod.gpu_ca.clone());
    drop(state_mod);

    Ok(response)
}

#[get("/gpu/get-iterations")]
async fn gpu_get_iterations(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let iterations = state_mod.gpu_ca.get_iteration_count();
    drop(state_mod);

    Ok(u32::to_string(&iterations))
}