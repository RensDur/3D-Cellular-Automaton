use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use super::super::appdata::dim3D::automaton::CellularAutomaton3D;

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

#[post("/initialise")]
pub async fn post_initialise(state: web::Data<Mutex<CellularAutomaton3D>>, info: web::Json<InfoPostInitialise>) -> impl Responder {
    let mut state_mod = state.lock().unwrap();
    state_mod.reset(
        info.size,
        info.dc_range,
        info.dc_influence,
        info.uc_range,
        info.uc_influence
    );
    drop(state_mod);

    HttpResponse::Ok()
}

#[post("/clear-all-voxels")]
pub async fn post_clear_all_voxels(state: web::Data<Mutex<CellularAutomaton3D>>) -> impl Responder {
    let mut state_mod = state.lock().unwrap();
    state_mod.clear_all_voxels();
    drop(state_mod);

    HttpResponse::Ok()
}

#[post("/spread-chemicals-randomly")]
pub async fn post_spread_chemicals_randomly(state: web::Data<Mutex<CellularAutomaton3D>>, info: web::Json<InfoPostSpreadChemicals>) -> impl Responder {
    let mut state_mod = state.lock().unwrap();
    state_mod.spread_chemicals_randomly(info.chemicals);
    drop(state_mod);

    HttpResponse::Ok()
}

#[post("/run-iteration")]
pub async fn post_run_iteration(state: web::Data<Mutex<CellularAutomaton3D>>) -> impl Responder {
    let mut state_mod = state.lock().unwrap();
    state_mod.run_iteration();
    drop(state_mod);

    HttpResponse::Ok()
}