use std::{sync::Mutex, cell::Cell};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use super::super::appdata::dim3D::automaton::CellularAutomaton3D;




#[get("/get-current-state")]
async fn get_current_state(state: web::Data<Mutex<CellularAutomaton3D>>) -> Result<impl Responder> {
    Ok(web::Json(state))
}

#[get("/reset-state")]
async fn get_reset_state(state: web::Data<Mutex<CellularAutomaton3D>>) -> impl Responder {
    let mut state_mod = state.lock().unwrap();
    state_mod.reset(50, 1.0, 1.0, 1.0, 1.0);
    drop(state_mod);

    HttpResponse::Ok()
}