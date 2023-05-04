use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use super::super::appdata::dim3D::automaton::CellularAutomaton3D;

#[derive(Deserialize)]
struct InfoPutInitialise {
    size: usize,
    dc_range: f32,
    dc_influence: f32,
    uc_range: f32,
    uc_influence: f32
}

#[post("/initialise")]
async fn put_initialise(info: web::Json<InfoPutInitialise>, state: web::Data<Mutex<CellularAutomaton3D>>) -> impl Responder {
    let mut state_mod = state.lock().unwrap();
    state_mod.reset(
        info.size,
        info.dc_range,
        info.dc_influence,
        info.uc_range,
        info.uc_influence
    );
    drop(state_mod);

    println!("Received /initialise request");

    HttpResponse::Ok()
}