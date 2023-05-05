use std::{sync::Mutex, cell::Cell};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use super::super::appdata::dim3D::automaton::CellularAutomaton3D;




#[get("/get-current-state")]
async fn get_current_state(state: web::Data<Mutex<CellularAutomaton3D>>) -> Result<impl Responder> {
    Ok(web::Json(state))
}