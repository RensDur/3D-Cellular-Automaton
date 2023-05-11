use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};
use crate::CAAppData;




#[get("/benchmarks/compare-cpu-gpu")]
async fn benchmarks_compare_cpu_gpu(state: web::Data<Mutex<CAAppData>>) -> Result<impl Responder> {

    let mut message = "Ok";
    
    let state_mod = state.lock().unwrap();

    // If cpu and gpu at a different number of iterations, make notice
    if state_mod.cpu_ca.iteration_count != state_mod.gpu_ca.iteration_count {
        message = format!("Generation mismatch: CPU[{}] versus GPU[{}]\nYou can request to run additional iterations until both simulations live in the same generation and compare them automatically.", state_mod.cpu_ca.iteration_count, state_mod.gpu_ca.iteration_count);
    }

    

}