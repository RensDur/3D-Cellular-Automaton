mod appdata;
mod routes;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use routes::{debug_routes::*, get::*, post::*};
use appdata::dim3d::automata::automaton_cpu::CPUCellularAutomaton3D;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(Mutex::new(CPUCellularAutomaton3D::new(20, 0.0, 0.0, 0.0, 0.0)));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard().allow_any_header().allow_any_method())
            .app_data(app_state.clone())
            .service(performance_check)
            .service(grid3d)
            .service(get_current_state)
            .service(post_initialise)
            .service(post_clear_all_voxels)
            .service(post_spread_chemicals_randomly)
            .service(post_run_iteration)
            
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await

}