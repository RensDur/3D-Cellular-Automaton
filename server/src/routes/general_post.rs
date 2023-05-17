use std::sync::Mutex;

use actix_web::{post, web, Responder, Result};
use crate::{CAAppData, appdata::dim3d::automata::automaton::CellularAutomaton3D, routes::cpu_post::{InfoPostSpreadChemicals, ResponsePostGeneral}};



/**
 * Method: randomly spread chemicals and make sure the CPU and GPU models get the same random state
 */
#[post("/general/spread-chemicals-randomly")]
async fn general_spread_chemicals_randomly(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostSpreadChemicals>) -> Result<impl Responder> {

    let mut state_mod = state.lock().unwrap();

    // Spread chemicals randomly on the CPU model
    state_mod.cpu_ca.spread_chemicals_randomly(info.chemicals);

    // Then copy this randomly spread state over to the GPU model
    let cpu_clone = state_mod.cpu_ca.clone();
    state_mod.gpu_ca.import_data_from_automaton(&cpu_clone);

    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))

}