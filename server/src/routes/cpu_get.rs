use std::sync::Mutex;

use actix_web::{get, web, Responder, Result, HttpRequest};
use serde::Deserialize;
use crate::{CAAppData, appdata::dim3d::automata::automaton::CellularAutomaton3D};


#[derive(Deserialize)]
pub struct ChunkInfo {
    split: usize,
    chunkx: usize,
    chunky: usize,
    chunkz: usize
}


#[get("/cpu/get-current-state")]
async fn cpu_get_current_state(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let response = web::Json(state_mod.cpu_ca.clone());
    drop(state_mod);

    Ok(response)
}

#[get("/cpu/get-current-state-triangles")]
async fn cpu_get_current_state_triangles(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();

    // Create a list of triangles according to the marching cubes algorithm
    let triangles = state_mod.cpu_ca.get_marching_cubes_mesh();

    drop(state_mod);

    Ok(triangles)
}

#[get("/cpu/get-current-state-triangles-chunk/{split}/{chunkx}/{chunky}/{chunkz}")]
async fn cpu_get_current_state_triangles_chunk(state: web::Data<Mutex<CAAppData>>, info: web::Path<ChunkInfo>) -> Result<impl Responder> {

    let split = info.split;
    let chunkx = info.chunkx;
    let chunky = info.chunky;
    let chunkz = info.chunkz;

    let state_mod = state.lock().unwrap();

    let ca_chunk = state_mod.cpu_ca.get_chunk(split, (chunkx, chunky, chunkz));

    // Create a gltf for this chunk and return it to the client
    let triangles = ca_chunk.get_marching_cubes_mesh();

    drop(state_mod);

    Ok(triangles)
}


#[get("/cpu/get-iterations")]
async fn cpu_get_iterations(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let iterations = state_mod.cpu_ca.get_iteration_count();
    drop(state_mod);

    Ok(u32::to_string(&iterations))
}