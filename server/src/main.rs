mod appdata;
mod routes;
mod gltfgeneration;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use routes::{debug_routes::*, cpu_get::*, cpu_post::*, gpu_get::*, gpu_post::*, general_post::*, benchmarks::{compare_cpu_gpu::{benchmarks_compare_cpu_gpu, benchmarks_compare_cpu_gpu_catch_up}, gpu_shader_increment::benchmarks_gpu_shader_increment}};
use appdata::dim3d::automata::{automaton_cpu::CPUCellularAutomaton3D, automaton::CellularAutomaton3D};
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

    let mut ca_app_data = CAAppData::new(3.2, 1.0, 6.0, -0.18);

    ca_app_data.gpu_ca.spread_chemicals_randomly(2);
    for _ in 0..50 {
        ca_app_data.gpu_ca.run_iteration();
    }

    let app_state = web::Data::new(Mutex::new(ca_app_data));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard().allow_any_header().allow_any_method())
            .app_data(app_state.clone())
            .service(performance_check)
            .service(grid3d)
            .service(cpu_get_current_state)
            .service(cpu_get_current_state_triangles)
            .service(cpu_get_iterations)
            .service(cpu_post_initialise)
            .service(cpu_post_clear_all_voxels)
            .service(cpu_post_spread_chemicals_randomly)
            .service(cpu_post_run_iteration)
            .service(gpu_get_current_state)
            .service(gpu_get_current_state_triangles)
            .service(gpu_get_iterations)
            .service(gpu_post_initialise)
            .service(gpu_post_clear_all_voxels)
            .service(gpu_post_spread_chemicals_randomly)
            .service(gpu_post_run_iteration)
            .service(general_spread_chemicals_randomly)
            .service(general_create_activator_patch)
            .service(benchmarks_compare_cpu_gpu)
            .service(benchmarks_compare_cpu_gpu_catch_up)
            .service(benchmarks_gpu_shader_increment)
            
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await

}