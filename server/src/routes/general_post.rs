use std::sync::Mutex;

use actix_web::{post, web, Responder, Result};
use serde::{Serialize, Deserialize};
use crate::{CAAppData, CAChemical, CAChemicalGroup, appdata::dim3d::automata::automaton::CellularAutomaton3D, routes::cpu_post::{InfoPostSpreadChemicals, ResponsePostGeneral}};











/**
 * Method: randomly spread chemicals and make sure the CPU and GPU models get the same random state
 */
#[post("/general/spread-chemicals-randomly")]
async fn general_spread_chemicals_randomly(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostSpreadChemicals>) -> Result<impl Responder> {

    let mut state_mod = state.lock().unwrap();

    // Spread chemicals randomly on the NCHEM model
    state_mod.nchem_ca.spread_chemicals_randomly(info.chemicals);

    // Then copy this randomly spread state over to the CPU and GPU models
    let nchem_clone = state_mod.nchem_ca.clone();
    state_mod.cpu_ca.import_data_from_automaton(&nchem_clone);
    state_mod.gpu_ca.import_data_from_automaton(&nchem_clone);

    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))

}


/**
 * Method:
 */
#[post("/general/create-activator-patch")]
async fn general_create_activator_patch(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let mut state_mod = state.lock().unwrap();


    // Create an activator patch
    state_mod.cpu_ca.clear_all_voxels();

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                state_mod.cpu_ca.set(x, y, z, 1);
            }
        }
    }

    // Then copy this randomly spread state over to the GPU model
    let cpu_clone = state_mod.cpu_ca.clone();
    state_mod.gpu_ca.import_data_from_automaton(&cpu_clone);

    state_mod.nchem_ca.import_data_from_automaton(&cpu_clone);

    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}