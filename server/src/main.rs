mod appdata;
mod routes;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use routes::{debug_routes::{performance_check, grid3d}, get::{get_current_state, get_reset_state}, put::put_initialise};
use appdata::dim3D::automaton::CellularAutomaton3D;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(Mutex::new(CellularAutomaton3D::new(20, 0.0, 0.0, 0.0, 0.0)));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard().allow_any_header().allow_any_method())
            .app_data(app_state.clone())
            .route("/initialise", web::post().to(put_initialise))
            .service(performance_check)
            .service(grid3d)
            .service(get_current_state)
            .service(get_reset_state)
            
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await

}