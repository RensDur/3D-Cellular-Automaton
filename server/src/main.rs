mod appdata;
mod routes;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use routes::{debug_routes::*, cpu_get::*, cpu_post::*, gpu_get::*, gpu_post::*};
use appdata::dim3d::automata::automaton_cpu::CPUCellularAutomaton3D;
use appdata::dim3d::automata::automaton_gpu::GPUCellularAutomaton3D;

use serde::{Serialize, Deserialize};

pub const AUTOMATON_SIZE: usize = 50;

#[derive(Clone, Serialize, Deserialize)]
pub struct CAAppData {
    pub cpu_ca: CPUCellularAutomaton3D,
    pub gpu_ca: GPUCellularAutomaton3D
}

impl CAAppData {
    pub fn new(dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32) -> Self {
        CAAppData {
            cpu_ca: CPUCellularAutomaton3D::new(AUTOMATON_SIZE, dc_range, dc_influence, uc_range, uc_influence),
            gpu_ca: GPUCellularAutomaton3D::new(dc_range, dc_influence, uc_range, uc_influence)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(Mutex::new(CAAppData::new(2.3, 1.0, 4.4, -0.19)));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard().allow_any_header().allow_any_method())
            .app_data(app_state.clone())
            .service(performance_check)
            .service(grid3d)
            .service(cpu_get_current_state)
            .service(cpu_post_initialise)
            .service(cpu_post_clear_all_voxels)
            .service(cpu_post_spread_chemicals_randomly)
            .service(cpu_post_run_iteration)
            .service(gpu_get_current_state)
            .service(gpu_post_initialise)
            .service(gpu_post_clear_all_voxels)
            .service(gpu_post_spread_chemicals_randomly)
            .service(gpu_post_run_iteration)
            
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await

}