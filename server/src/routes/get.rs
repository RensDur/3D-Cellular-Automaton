use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use super::super::appdata::dim3d::automaton::CellularAutomaton3D;




#[get("/get-current-state")]
async fn get_current_state(state: web::Data<Mutex<CellularAutomaton3D>>) -> Result<impl Responder> {
    Ok(web::Json(state))
}