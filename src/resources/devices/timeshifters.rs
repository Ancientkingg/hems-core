use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::scope;

use crate::api::demkit::{self, env::{InternalComplex, TimeShifterEntityParams}, timeshifters::{Job, ScheduleJob, TimeShifters}, Measurement};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        scope::scope("/timeshifters/{entity_name}")
            .service(get_by_id)
            .service(add_by_id)
            .service(remove_by_id)
            .service(schedule_job)
            .service(cancel_job)
            .service(force_shutdown),
    );
}

#[derive(Serialize, ToSchema)]
struct DeviceStatus {
    /// House ID
    house_id: u32,
    /// Name of the timeshifter entity
    entity_name: String,
    /// Indicates if the timeshifter is active
    is_active: bool,
    /// The currently active job, if any
    #[schema(nullable)]
    active_job: Option<Job>,
    /// The progress of the current job as a percentage
    #[schema(value_type = f64, minimum = 0.0, maximum = 100.0)]
    progress: f64,
    /// The index of the currently active job
    active_job_idx: i32,
    /// The list of scheduled jobs
    scheduled_jobs: Vec<Job>,
    /// The current electricity consumption of the timeshifter
    consumption: Measurement,
    /// The device profile of the timeshifter as a list of complex numbers
    profile: Vec<InternalComplex>
}
#[utoipa::path(
    get,
    tag = "Timeshifters",
    description = "Get timeshifter properties.",
    responses(
        (status = 200, description = "Get timeshifter properties", body = DeviceStatus),
        (status = 400, description = "Invalid entity name"),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the timeshifter entity"),
    )
)]
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
        profile: device_profile
            .iter()
            .map(|complex| InternalComplex::from(*complex))
            .collect(),
    };

    HttpResponse::Ok().json(device_status)
}


#[utoipa::path(
    post,
    tag = "Timeshifters",
    description = "Add a new timeshifter entity.",
    responses(
        (status = 200, description = "Successfully added timeshifter entity", body = String),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the timeshifter entity"),
    ),
    request_body = TimeShifterEntityParams,
)]
#[post("")]
async fn add_by_id(id: web::Path<(u32, String)>, params: web::Json<TimeShifterEntityParams>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::add_timeshifter(house_id, params.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} added successfully")),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    HttpResponse::Ok().body(format!("{entity_name} added successfully"))
}

#[utoipa::path(
    delete,
    tag = "Timeshifters",
    description = "Remove a timeshifter entity from the house.",
    responses(
        (status = 200, description = "Successfully removed timeshifter entity", body = String),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the timeshifter entity"),
    ),
)]
#[delete("")]
async fn remove_by_id(id: web::Path<(u32, String)>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::remove_entity(house_id, entity_name.as_str()).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} removed successfully")),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[utoipa::path(
    post,
    tag = "Timeshifters",
    description = "Schedule a job for a timeshifter entity",
    responses(
        (status = 200, description = "Successfully scheduled job for timeshifter entity", body = Job),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the timeshifter entity"),
    ),
    request_body = ScheduleJob,
)]
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

#[utoipa::path(
    delete,
    tag = "Timeshifters",
    description = "Cancel a scheduled job for a timeshifter entity",
    responses(
        (status = 200, description = "Successfully scheduled job for timeshifter entity", body = String),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the timeshifter entity"),
        ("job_id" = u32, description = "Job ID to cancel"),
    ),
)]
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

#[utoipa::path(
    post,
    tag = "Timeshifters",
    description = "Force shutdown timeshifter entity immediately, potentially canceling and discarding any active jobs.",
    responses(
        (status = 200, description = "Successfully shut down timeshifter entity", body = String),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the timeshifter entity"),
    ),
)]
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
