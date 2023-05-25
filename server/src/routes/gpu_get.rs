use std::sync::Mutex;

use actix_web::{get, web, Responder, Result, HttpRequest};
use serde::Deserialize;
use crate::appdata::dim3d::automata::automaton::CellularAutomaton3D;
use crate::appdata::dim3d::automata::automaton_cpu::CPUCellularAutomaton3D;
use crate::appdata::dim3d::automata::automaton_gpu::GPUCellularAutomaton3D;
use crate::CAAppData;

#[derive(Deserialize)]
pub struct InfoGetChunk {
    split: usize,
    chunkx: usize,
    chunky: usize,
    chunkz: usize
}

#[get("/gpu/get-current-state")]
async fn gpu_get_current_state(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let response = web::Json(state_mod.gpu_ca.clone());
    drop(state_mod);

    Ok(response)
}

#[get("/gpu/get-current-state-triangles")]
async fn gpu_get_current_state_triangles(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();

    // Create a list of triangles according to the marching cubes algorithm
    let triangles = state_mod.gpu_ca.get_marching_cubes_mesh();

    drop(state_mod);

    Ok(triangles)
}

#[get("/gpu/get-current-state-triangles-chunk{split}/{chunkx}/{chunky}/{chunkz}")]
async fn gpu_get_current_state_triangles_chunk(state: web::Data<Mutex<CAAppData>>, req: HttpRequest) -> Result<impl Responder> {

    let split = req.match_info().query("split").parse().unwrap();
    let chunkx = req.match_info().query("chunkx").parse().unwrap();
    let chunky = req.match_info().query("chunky").parse().unwrap();
    let chunkz = req.match_info().query("chunkz").parse().unwrap();

    let state_mod = state.lock().unwrap();

    let ca_chunk = state_mod.gpu_ca.get_chunk(split, (chunkx, chunky, chunkz));

    // Create a gltf for this chunk and return it to the client
    let triangles = ca_chunk.get_marching_cubes_mesh();

    drop(state_mod);

    Ok(triangles)
}

#[get("/gpu/get-iterations")]
async fn gpu_get_iterations(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let state_mod = state.lock().unwrap();
    let iterations = state_mod.gpu_ca.get_iteration_count();
    drop(state_mod);

    Ok(u32::to_string(&iterations))
}