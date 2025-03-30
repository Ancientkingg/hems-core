use actix_web::{delete, get, post, web, HttpResponse, Responder};
use num_complex::Complex;
use serde::Serialize;

use crate::api::demkit::{self, env::TimeShifterEntityParams, timeshifters::{Job, ScheduleJob, TimeShifters}, Measurement};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/timeshifters/{id}")
            .service(get_by_id)
            .service(add_by_id)
            .service(remove_by_id)
            .service(schedule_job)
            .service(cancel_job)
            .service(force_shutdown),
    );
}

#[derive(Serialize)]
struct DeviceStatus {
    house_id: u32,
    entity_name: String,
    is_active: bool,
    active_job: Option<Job>,
    progress: f64,
    active_job_idx: i32,
    scheduled_jobs: Vec<Job>,
    consumption: Measurement,
    profile: Vec<Complex<f64>>
}

#[get("")]
async fn get_by_id(id: web::Path<(u32, String)>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    let timeshifter = match TimeShifters::try_from(entity_name.as_str()) {
        Ok(ts) => ts,
        Err(e) => return HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    };

    let device_properties = match demkit::timeshifters::get_properties(house_id, timeshifter).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let device_profile = device_properties.device_profile.unwrap();
    let profile_length = device_profile.len();

    let device_status = DeviceStatus {
        house_id,
        entity_name: device_properties.name,
        is_active: device_properties.available,
        active_job: if device_properties.available { device_properties.current_job } else { None },
        active_job_idx: device_properties.current_job_idx,
        progress: device_properties.job_progress / (profile_length as f64) * 100.0,
        scheduled_jobs: device_properties.jobs,
        consumption: Measurement {
            value: device_properties.electricity_consumption.unwrap().norm(),
            unit: "W".to_string(),
        },
        profile: device_profile,
    };

    HttpResponse::Ok().json(device_status)
}

#[post("")]
async fn add_by_id(id: web::Path<(u32, String)>, params: web::Json<TimeShifterEntityParams>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::add_timeshifter(house_id, params.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} added successfully")),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    HttpResponse::Ok().body(format!("{entity_name} added successfully"))
}

#[delete("")]
async fn remove_by_id(id: web::Path<(u32, String)>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::remove_entity(house_id, entity_name.as_str()).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} removed successfully")),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[post("/job")]
async fn schedule_job(id: web::Path<(u32, String)>, body: web::Json<ScheduleJob>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    let timeshifter = match TimeShifters::try_from(entity_name.as_str()) {
        Ok(ts) => ts,
        Err(e) => return HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    };

    match demkit::timeshifters::schedule_job(house_id, timeshifter, body.into_inner()).await {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[delete("/job/{id}")]
async fn cancel_job(id: web::Path<(u32, String, u32)>) -> impl Responder {
    let (house_id, entity_name, job_id) = id.into_inner();

    let timeshifter = match TimeShifters::try_from(entity_name.as_str()) {
        Ok(ts) => ts,
        Err(e) => return HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    };

    match demkit::timeshifters::cancel_job(house_id, timeshifter, job_id).await {
        Ok(_) => HttpResponse::Ok().body(format!("Job {job_id} cancelled for {entity_name}")),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[get("/shutdown")]
async fn force_shutdown(id: web::Path<(u32, String)>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    let timeshifter = match TimeShifters::try_from(entity_name.as_str()) {
        Ok(ts) => ts,
        Err(e) => return HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    };

    match demkit::timeshifters::force_shutdown(house_id, timeshifter).await {
        Ok(_) => HttpResponse::Ok().body(format!("Shutdown successful for {entity_name}")),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
