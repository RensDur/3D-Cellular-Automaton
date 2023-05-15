use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::{CAAppData, appdata::dim3d::automata::automaton::CellularAutomaton3D};




#[get("/cpu/get-current-state")]
async fn cpu_get_current_state(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let response = web::Json(state_mod.cpu_ca.clone());
    drop(state_mod);

    Ok(response)
}

#[get("/cpu/get-iterations")]
async fn cpu_get_iterations(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let iterations = state_mod.cpu_ca.get_iteration_count();
    drop(state_mod);

    Ok(u32::to_string(&iterations))
}