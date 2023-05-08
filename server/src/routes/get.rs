use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::appdata::dim3d::automata::automaton_cpu::CPUCellularAutomaton3D;




#[get("/get-current-state")]
async fn get_current_state(state: web::Data<Mutex<CPUCellularAutomaton3D>>) -> Result<impl Responder> {
    Ok(web::Json(state))
}