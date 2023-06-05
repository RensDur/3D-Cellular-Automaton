use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::appdata::dim3d::automata::automaton::CellularAutomaton3D;
use crate::CAAppData;



#[get("/nchem/get-current-state")]
async fn nchem_get_current_state(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let response = web::Json(state_mod.nchem_ca.clone());
    drop(state_mod);

    Ok(response)
}

#[get("/nchem/get-current-state-triangles")]
async fn nchem_get_current_state_triangles(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();

    // Create a list of triangles according to the marching cubes algorithm
    let triangles = state_mod.nchem_ca.get_marching_cubes_mesh();

    drop(state_mod);

    Ok(triangles)
}

#[get("/nchem/get-iterations")]
async fn nchem_get_iterations(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let iterations = state_mod.nchem_ca.get_iteration_count();
    drop(state_mod);

    Ok(u32::to_string(&iterations))
}


#[get("/nchem/get-chemical-capture")]
async fn nchem_get_chemical_capture(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let state_mod = state.lock().unwrap();
    let chemical_capture = state_mod.nchem_ca.get_captured_chemical();
    drop(state_mod);

    Ok(usize::to_string(&chemical_capture))

}