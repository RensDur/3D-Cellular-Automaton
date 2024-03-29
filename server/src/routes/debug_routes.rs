use actix_web::{get, web, HttpResponse, Responder, Result};

use crate::appdata::dim3d::grid::CAGrid3D;


#[get("/performance-check")]
async fn performance_check() -> impl Responder {
    HttpResponse::Ok().body("Performance nominal.")
}


#[get("/grid3d")]
async fn grid3d() -> Result<impl Responder> {
    Ok(web::Json(CAGrid3D::new(2)))
}