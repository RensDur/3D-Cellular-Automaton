use std::{sync::Mutex, time::Instant};

use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};
use super::super::appdata::dim3d::automaton::CellularAutomaton3D;

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

#[post("/initialise")]
pub async fn post_initialise(state: web::Data<Mutex<CellularAutomaton3D>>, info: web::Json<InfoPostInitialise>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.reset(
        info.size,
        info.dc_range,
        info.dc_influence,
        info.uc_range,
        info.uc_influence
    );
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/clear-all-voxels")]
pub async fn post_clear_all_voxels(state: web::Data<Mutex<CellularAutomaton3D>>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.clear_all_voxels();
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/spread-chemicals-randomly")]
pub async fn post_spread_chemicals_randomly(state: web::Data<Mutex<CellularAutomaton3D>>, info: web::Json<InfoPostSpreadChemicals>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.spread_chemicals_randomly(info.chemicals);
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/run-iteration")]
pub async fn post_run_iteration(state: web::Data<Mutex<CellularAutomaton3D>>, info: web::Json<InfoPostRunIteration>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    
    let start = Instant::now();

    for _ in 0..info.num_iterations {
        state_mod.run_iteration();
    }

    let duration = start.elapsed();

    drop(state_mod);

    Ok(web::Json(ResponsePostRunIteration{duration: duration.as_secs_f32()}))
}