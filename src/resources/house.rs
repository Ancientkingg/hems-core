use std::vec;

use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[path = "devices/devices.rs"]
mod devices;
use devices::{battery, ha_entity, meter, solar, thermal, timeshifters};

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/houses/{id}")
            .service(get_by_id)
            .service(get_time)
            .service(compose)
            .service(reset)
            .service(load)
            .service(set_config)
            .service(list_entities)
            .configure(battery::configure)
            .configure(meter::configure)
            .configure(solar::configure)
            .configure(thermal::configure)
            .configure(timeshifters::configure)
            .configure(ha_entity::configure),
    );
}

#[get("")]
async fn get_by_id(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("House id: {}", path))
    // list entities and show /composer
}

#[post("")]
async fn compose(path: web::Path<u32>) -> impl Responder {
    let house_id = path.into_inner();
    match demkit::env::add_host(house_id).await {
        Ok(_) => println!("Host added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    match demkit::env::add_weather(house_id).await {
        Ok(_) => println!("Weather added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    match demkit::env::add_sun(house_id).await {
        Ok(_) => println!("Sun added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let sm_params = demkit::env::MeterEntityParams {
        name: "SmartMeter".to_string(),
        commodities: vec!["ELECTRICITY".to_string()],
        weights: vec![("ELECTRICITY".to_string(), 1.0)],
    };

    match demkit::env::add_meter(house_id, sm_params).await {
        Ok(_) => println!("Meter added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let gm_params = demkit::env::MeterEntityParams {
        name: "SmartGasMeter".to_string(),
        commodities: vec!["NATGAS".to_string()],
        weights: vec![("NATGAS".to_string(), 1.0)],
    };

    match demkit::env::add_meter(house_id, gm_params).await {
        Ok(_) => println!("Meter added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let curt_params = demkit::env::CurtEntityParams {
        name: "Load".to_string(),
        filename: "sampledata/singlehouse/Electricity_Profile.csv".to_string(),
        filename_reactive: "sampledata/singlehouse/Reactive_Electricity_Profile.csv".to_string(),
        column: house_id as u64,
        time_base: 60,
    };

    match demkit::env::add_curt(house_id, curt_params).await {
        Ok(_) => println!("Curt added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let zone_params = demkit::env::ZoneEntityParams {
        name: "Zone".to_string(),
        r_floor: 0.001,
        r_envelope: 0.0064,
        c_floor: 5100.0 * 3600.0,
        c_zone: 21100.0 * 3600.0,
        initial_temperature: 18.5,
    };

    match demkit::env::add_zone(house_id, zone_params).await {
        Ok(_) => println!("Zone added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let thermostat_params = demkit::env::ThermostatEntityParams {
        name: "Thermostat".to_string(),
        temperature_setpoint_heating: 21.0,
        temperature_setpoint_cooling: 23.0,
        temperature_min: 21.0,
        temperature_max: 23.0,
        temperature_deadband: vec![-0.1, 0.0, 0.5, 0.6],
        preheating_time: 3600.0,
    };

    match demkit::env::add_thermostat(house_id, thermostat_params).await {
        Ok(_) => println!("Thermostat added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let dhw_params = demkit::env::DhwEntityParams {
        name: "DomesticHotWater".to_string(),
    };

    match demkit::env::add_dhw(house_id, dhw_params).await {
        Ok(_) => println!("DHW added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let heat_source_params = demkit::env::HeatSourceEntityParams {
        name: "HeatPump".to_string(),
        // producing_temperatures: vec![0.0, 35.0],
        // producing_powers: vec![0.0, 4500.0]
    };

    match demkit::env::add_heat_source(house_id, heat_source_params).await {
        Ok(_) => println!("Heat Source added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    let heat_pump_params = demkit::env::HeatPumpEntityParams {
        name: "DomesticHotWaterControllerBoiler".to_string(),
        producing_temperatures: vec![0.0, 60.0],
        producing_powers: vec![0.0, 25000.0]
    };

    match demkit::env::add_heat_pump(house_id, heat_pump_params).await {
        Ok(_) => println!("Heat Pump added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    HttpResponse::Ok().body("House composed successfully")
}


#[delete("")]
async fn reset(path: web::Path<u32>) -> impl Responder {
    let _house_id = path.into_inner();
    match demkit::env::reset().await {
        Ok(_) => println!("House reset successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }

    HttpResponse::Ok().body("House simulation reset successfully")
}

#[post("/load")]
async fn load(path: web::Path<u32>) -> impl Responder {
    let _house_id = path.into_inner();
    match demkit::env::load().await {
        Ok(_) => println!("House loaded successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }

    match demkit::env::start().await {
        Ok(_) => println!("House started successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }

    HttpResponse::Ok().body("House simulation loaded successfully and currently running")
}

#[post("/config")]
async fn set_config(_path: web::Path<u32>, config: web::Json<demkit::env::SimConfig>) -> impl Responder {
    match demkit::env::set_config(config.into_inner()).await {
        Ok(_) => println!("House config set successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    };

    HttpResponse::Ok().body("House config set successfully")
}

#[get("/time")]
async fn get_time() -> impl Responder {
    let current_time = demkit::get_time().await;
    HttpResponse::Ok().body(current_time.to_string())
}

#[get("/entities")]
async fn list_entities() -> impl Responder {
    let entities = demkit::list_entities().await;
    HttpResponse::Ok().json(entities)
}