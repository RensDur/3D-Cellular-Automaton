use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::appdata::dim3d::automata::automaton_cpu::CPUCellularAutomaton3D;
use crate::appdata::dim3d::automata::automaton_gpu::GPUCellularAutomaton3D;
use crate::CAAppData;




#[get("/cpu/get-current-state")]
async fn cpu_get_current_state(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let response = web::Json(state_mod.cpu_ca.clone());
    drop(state_mod);

    Ok(response)
}