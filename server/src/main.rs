mod appdata;
mod routes;
mod gltfgeneration;

use std::{sync::Mutex, time::Instant};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use routes::{debug_routes::*, cpu_get::*, cpu_post::*, gpu_get::*, gpu_post::*, nchem_get::*, nchem_post::*, general_get::*, general_post::*, batch::*, benchmarks::{compare_cpu_gpu::{benchmarks_compare_cpu_gpu, benchmarks_compare_cpu_gpu_catch_up}, gpu_shader_increment::benchmarks_gpu_shader_increment}};
use appdata::dim3d::automata::{automaton_cpu::CPUCellularAutomaton3D, automaton::CellularAutomaton3D};
use appdata::dim3d::automata::automaton_gpu::GPUCellularAutomaton3D;
use appdata::dim3d::automata::automaton_gpu_n_chemicals::{GPUNChemicalsCellularAutomaton3D, CAChemicalGroup, CAChemical};

use serde::{Serialize, Deserialize};

pub const AUTOMATON_SIZE: usize = 30;
pub const K_MAX: usize = 20;

#[derive(Clone, Serialize, Deserialize)]
pub struct CAAppData {
    pub cpu_ca: CPUCellularAutomaton3D,
    pub gpu_ca: GPUCellularAutomaton3D,
    pub nchem_ca: GPUNChemicalsCellularAutomaton3D
}

impl CAAppData {
    pub fn new(dc_range: f32, dc_influence: f32, uc_range: f32, uc_influence: f32, chemicals: Vec<CAChemicalGroup>) -> Self {
        CAAppData {
            cpu_ca: CPUCellularAutomaton3D::new(AUTOMATON_SIZE, dc_range, dc_influence, uc_range, uc_influence),
            gpu_ca: GPUCellularAutomaton3D::new(dc_range, dc_influence, uc_range, uc_influence),
            nchem_ca: GPUNChemicalsCellularAutomaton3D::new(chemicals)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let chemicals = vec![
        // CAChemicalGroup { // 0
        //     promote: CAChemical {
        //         range: 9.0,
        //         influence: 1.0
        //     },
        //     demote: CAChemical {
        //         range: 14.0,
        //         influence: -0.35
        //     }
        // },
        // CAChemicalGroup { // 1
        //     promote: CAChemical {
        //         range: 5.0,
        //         influence: 1.0
        //     },
        //     demote: CAChemical {
        //         range: 9.4,
        //         influence: -0.2
        //     }
        // },
        // CAChemicalGroup { // 2
        //     promote: CAChemical {
        //         range: 3.2,
        //         influence: 1.0
        //     },
        //     demote: CAChemical {
        //         range: 6.0,
        //         influence: -0.2
        //     }
        // },
        // CAChemicalGroup { // 3
        //     promote: CAChemical {
        //         range: 2.0,
        //         influence: 2.0
        //     },
        //     demote: CAChemical {
        //         range: 3.0,
        //         influence: -0.34
        //     }
        // },
        // CAChemicalGroup { // 4
        //     promote: CAChemical {
        //         range: 3.2,
        //         influence: 1.0
        //     },
        //     demote: CAChemical {
        //         range: 6.0,
        //         influence: -0.24
        //     }
        // },
        // CAChemicalGroup { // 5
        //     promote: CAChemical {
        //         range: 3.2,
        //         influence: 1.0
        //     },
        //     demote: CAChemical {
        //         range: 6.0,
        //         influence: -0.2
        //     }
        // },
        // CAChemicalGroup { // 6
        //     promote: CAChemical {
        //         range: 3.2,
        //         influence: 1.0
        //     },
        //     demote: CAChemical {
        //         range: 6.0,
        //         influence: -0.2
        //     }
        // },
        // CAChemicalGroup { // 6
        //     promote: CAChemical {
        //         range: 3.2,
        //         influence: 1.0
        //     },
        //     demote: CAChemical {
        //         range: 6.0,
        //         influence: -0.2
        //     }
        // },
        CAChemicalGroup { // 6
            promote: CAChemical {
                range: 3.2,
                influence: 1.0
            },
            demote: CAChemical {
                range: 6.0,
                influence: -0.2
            }
        }
    ];

    let mut ca_app_data = CAAppData::new(3.2, 1.0, 6.0, -0.2, chemicals);

    // ca_app_data.nchem_ca.spread_chemicals_randomly(8);
    // for _ in 0..100 {
    //     ca_app_data.nchem_ca.run_iteration();
    // }

    println!("Done!");


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
            .service(nchem_get_current_state)
            .service(nchem_get_current_state_triangles)
            .service(nchem_get_iterations)
            .service(nchem_get_chemical_capture)
            .service(nchem_get_order_parameter)
            .service(nchem_get_species_configuration)
            .service(nchem_post_initialise)
            .service(nchem_post_clear_all_voxels)
            .service(nchem_post_spread_chemicals_randomly)
            .service(nchem_post_run_iteration)
            .service(nchem_post_set_chemical_capture)
            .service(nchem_set_species_configuration)
            .service(general_get_automaton_size)
            .service(general_spread_chemicals_randomly)
            .service(general_create_activator_patch)
            .service(benchmarks_compare_cpu_gpu)
            .service(benchmarks_compare_cpu_gpu_catch_up)
            .service(benchmarks_gpu_shader_increment)
            .service(batch_run_experiment)
            
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await

}