use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::appdata::dim3d::automata::automaton::CellularAutomaton3D;
use crate::CAAppData;



#[get("/dim2d/get-current-state")]
async fn dim2d_get_current_state(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let response = web::Json(state_mod.dim2d_ca.clone());
    drop(state_mod);

    Ok(response)
}

#[get("/dim2d/get-current-state-triangles")]
async fn dim2d_get_current_state_triangles(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();

    // Create a list of triangles according to the marching cubes algorithm
    let triangles = state_mod.dim2d_ca.get_marching_cubes_mesh();

    drop(state_mod);

    Ok(triangles)
}

#[get("/dim2d/get-iterations")]
async fn dim2d_get_iterations(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let iterations = state_mod.dim2d_ca.get_iteration_count();
    drop(state_mod);

    Ok(u32::to_string(&iterations))
}


#[get("/dim2d/get-chemical-capture")]
async fn dim2d_get_chemical_capture(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let state_mod = state.lock().unwrap();
    let chemical_capture = state_mod.dim2d_ca.get_captured_chemical();
    drop(state_mod);

    Ok(usize::to_string(&chemical_capture))

}

#[get("/dim2d/get-order-parameter")]
async fn dim2d_get_order_parameter(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let state_mod = state.lock().unwrap();

    let result = state_mod.dim2d_ca.get_order_parameters();

    drop(state_mod);

    Ok(web::Json(result))
}

#[get("/dim2d/get-species-configuration")]
async fn dim2d_get_species_configuration(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let state_mod = state.lock().unwrap();

    let result = state_mod.dim2d_ca.chemicals.clone();

    drop(state_mod);

    Ok(web::Json(result))

}