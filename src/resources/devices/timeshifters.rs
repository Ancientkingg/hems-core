use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/timeshifters/{id}")
            .service(get_by_id)
            .service(schedule_job)
            .service(cancel_job),
    );
}

#[derive(Serialize)]
struct JobInfo {
    house_id: u32,
    job_id: u32,
    start_time: u64,
    end_time: u64,
    device: String,
}

#[derive(Serialize)]
struct DeviceStatus {
    house_id: u32,
    entity_name: String,
    is_active: bool,
    active_job: Option<Job>,
    progress: f64,
    scheduled_jobs: Vec<Job>,
    consumption: f64,
}

#[get("")]
async fn get_by_id(id: web::Path<(u32, String, u32)>) -> impl Responder {
    let (house_id, entity_name, job_id) = id.into_inner();

    todo!("")
}

#[post("/job")]
async fn schedule_job(id: web::Path<(u32, String)>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    todo!("")
}

#[delete("/job")]
async fn cancel_job(id: web::Path<(u32, String, u32)>) -> impl Responder {
    let (house_id, entity_name, _job_id) = id.into_inner();

    todo!("")
}

async fn force_shutdown(id: web::Path<u32, String>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    todo!("")
}
