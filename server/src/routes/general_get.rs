
use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::{CAAppData, AUTOMATON_SIZE};







#[get("/general/get-automaton-size")]
async fn general_get_automaton_size(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    Ok(web::Json(AUTOMATON_SIZE))

}



