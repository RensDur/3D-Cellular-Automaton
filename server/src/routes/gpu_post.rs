use std::{sync::Mutex, time::Instant};

use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};
use crate::appdata::dim3d::automata::{automaton::CellularAutomaton3D, automaton_cpu::CPUCellularAutomaton3D, automaton_gpu::GPUCellularAutomaton3D};
use crate::CAAppData;

#[derive(Deserialize)]
pub struct InfoPostInitialise {
    size: usize,
    dc_range: f32,
    dc_influence: f32,
    uc_range: f32,
    uc_influence: f32
}

#[derive(Deserialize)]
pub struct InfoPostSpreadChemicals {
    chemicals: u32
}

#[derive(Deserialize)]
pub struct InfoPostRunIteration {
    num_iterations: u32
}

#[derive(Serialize)]
pub struct ResponsePostGeneral {
    status: u32
}

#[derive(Serialize)]
pub struct ResponsePostRunIteration {
    duration: f32
}

#[post("/gpu/initialise")]
pub async fn gpu_post_initialise(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostInitialise>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.gpu_ca.reset(
        info.size,
        info.dc_range,
        info.dc_influence,
        info.uc_range,
        info.uc_influence
    );
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/gpu/clear-all-voxels")]
pub async fn gpu_post_clear_all_voxels(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.gpu_ca.clear_all_voxels();
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/gpu/spread-chemicals-randomly")]
pub async fn gpu_post_spread_chemicals_randomly(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostSpreadChemicals>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.gpu_ca.spread_chemicals_randomly(info.chemicals);
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/gpu/run-iteration")]
pub async fn gpu_post_run_iteration(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostRunIteration>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    
    let start = Instant::now();

    for _ in 0..info.num_iterations {
        state_mod.gpu_ca.run_iteration();
    }

    let duration = start.elapsed();

    drop(state_mod);

    Ok(web::Json(ResponsePostRunIteration{duration: duration.as_secs_f32()}))
}