use std::{sync::Mutex, time::Instant};

use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};
use crate::appdata::dim3d::automata::automaton::CellularAutomaton3D;
use crate::{CAAppData, CAChemical, CAChemicalGroup};

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

#[derive(Deserialize)]
pub struct InfoPostSetChemicalCapture {
    chemical_capture: usize
}

#[derive(Serialize)]
pub struct ResponsePostGeneral {
    status: u32
}

#[derive(Serialize)]
pub struct ResponsePostRunIteration {
    duration: f32
}




#[derive(Serialize, Deserialize)]
pub struct InfoPostChemicalHelper {
    range: f32,
    influence: f32
}

#[derive(Serialize, Deserialize)]
pub struct InfoPostSpeciesHelper {
    chemicalA: InfoPostChemicalHelper,
    chemicalB: InfoPostChemicalHelper
}

#[derive(Serialize, Deserialize)]
pub struct InfoPostSetSpeciesConfiguration {
    species: Vec<InfoPostSpeciesHelper>
}


#[post("/dim2d/initialise")]
pub async fn dim2d_post_initialise(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostInitialise>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.dim2d_ca.reset(
        info.size,
        info.dc_range,
        info.dc_influence,
        info.uc_range,
        info.uc_influence
    );
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/dim2d/clear-all-voxels")]
pub async fn dim2d_post_clear_all_voxels(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.dim2d_ca.clear_all_voxels();
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/dim2d/spread-chemicals-randomly")]
pub async fn dim2d_post_spread_chemicals_randomly(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostSpreadChemicals>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    state_mod.dim2d_ca.spread_chemicals_randomly(info.chemicals);
    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
}

#[post("/dim2d/run-iteration")]
pub async fn dim2d_post_run_iteration(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostRunIteration>) -> Result<impl Responder> {
    let mut state_mod = state.lock().unwrap();
    
    let start = Instant::now();

    for _ in 0..info.num_iterations {
        state_mod.dim2d_ca.run_iteration();
    }

    let duration = start.elapsed();

    drop(state_mod);

    Ok(web::Json(ResponsePostRunIteration{duration: duration.as_secs_f32()}))
}


#[post("/dim2d/set-chemical-capture")]
pub async fn dim2d_post_set_chemical_capture(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostSetChemicalCapture>) -> Result<impl Responder> {

    let mut state_mod = state.lock().unwrap();

    state_mod.dim2d_ca.capture_chemical(info.chemical_capture);

    println!("Captured chemical!");

    drop(state_mod);

    Ok(web::Json(ResponsePostGeneral{status: 0}))
    
}


#[post("/dim2d/set-species-configuration")]
async fn dim2d_set_species_configuration(state: web::Data<Mutex<CAAppData>>, info: web::Json<InfoPostSetSpeciesConfiguration>) -> Result<impl Responder> {

    // Construct the correct list of chemicals
    let mut chemicals: Vec<CAChemicalGroup> = vec![];

    for s in &info.species {
        chemicals.push(CAChemicalGroup {
            promote: CAChemical {
                range: s.chemicalA.range,
                influence: s.chemicalA.influence
            },
            demote: CAChemical {
                range: s.chemicalB.range,
                influence: s.chemicalB.influence
            }
        });
    }

    // Set the new chemicals array
    let mut state_mod = state.lock().unwrap();
    state_mod.dim2d_ca.chemicals = chemicals;
    drop(state_mod);

    Ok("")
}