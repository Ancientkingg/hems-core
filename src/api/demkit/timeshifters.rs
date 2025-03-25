use std::fmt;

use num_complex::Complex;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use super::{init, parse_complex_str, ApiError, Commodities, Commodity, BASE_URL, CLIENT};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub start_time: u64,
    pub end_time: u64,
}

fn deserialize_jobs<'de, D>(deserializer: D) -> Result<Vec<Job>, D::Error>
where
    D: Deserializer<'de>,
{
    struct JobVisitor;

    impl<'de> Visitor<'de> for JobVisitor {
        type Value = Vec<Job>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "an array with an integer ID and an object containing startTime and endTime",
            )
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut jobs: Vec<Job> = Vec::new();

            while let Some(job) = seq.next_element::<(u64, Job)>()? {
                jobs.push(Job {
                    start_time: job.1.start_time,
                    end_time: job.1.end_time,
                });
            }

            Ok(jobs)
        }
    }

    deserializer.deserialize_seq(JobVisitor)
}

// Custom deserializer function
fn deserialize_empty_as_none<'de, D>(deserializer: D) -> Result<Option<Job>, D::Error>
where
    D: Deserializer<'de>,
{
    struct EmptyAsNone;

    impl<'de> Visitor<'de> for EmptyAsNone {
        type Value = Option<Job>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct or an empty object `{}`")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            // parse Job { startTime: 0, endTime: 0 }
            let mut start_time: Option<u64> = None;
            let mut end_time: Option<u64> = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "startTime" => {
                        start_time = Some(map.next_value()?);
                    }
                    "endTime" => {
                        end_time = Some(map.next_value()?);
                    }
                    _ => {
                        let _: serde_json::Value = map.next_value()?;
                    }
                }
            }

            match (start_time, end_time) {
                (None, None) => Ok(None),
                (None, _) => Err(de::Error::custom("startTime field not found")),
                (_, None) => Err(de::Error::custom("endTime field not found")),
                (Some(start_time), Some(end_time)) => Ok(Some(Job { start_time, end_time })),
            }
        }
    }

    deserializer.deserialize_any(EmptyAsNone)
}

pub enum TimeShifters {
    DishWasher,
    WashingMachine,
}

impl TimeShifters {
    pub fn get_device_name(&self) -> &str {
        match self {
            TimeShifters::DishWasher => "DishWasher",
            TimeShifters::WashingMachine => "WashingMachine",
        }
    }
}

impl TryFrom<&str> for TimeShifters {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "DishWasher" => Ok(TimeShifters::DishWasher),
            "WashingMachine" => Ok(TimeShifters::WashingMachine),
            _ => Err("Invalid TimeShifter name"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TimeShifterInfo {
    pub name: String,
    #[serde(rename = "consumption")]
    _consumption: Commodities,
    pub electricity_consumption: Option<Complex<f64>>,
    #[serde(rename = "profile")]
    _profile: Vec<String>,
    pub device_profile: Option<Vec<Complex<f64>>>,
    pub available: bool,
    #[serde(rename = "currentJob")]
    #[serde(deserialize_with = "deserialize_empty_as_none")]
    pub current_job: Option<Job>,
    #[serde(rename = "currentJobIdx")]
    pub current_job_idx: i32,
    #[serde(deserialize_with = "deserialize_jobs")]
    pub jobs: Vec<Job>,
    #[serde(rename = "jobProgress")]
    pub job_progress: f64,
}

#[derive(Deserialize)]
pub struct ScheduleJob {
    delay: u64,
    duration: u64,
}

pub async fn get_properties(
    house_id: u32,
    entity: TimeShifters,
) -> Result<TimeShifterInfo, ApiError> {
    let client = CLIENT.get_or_init(init);

    let entity_name = entity.get_device_name();
    let entity_id = format!("{entity_name}-House-{house_id}");

    let url = format!("{}/call/{entity_id}/getProperties", BASE_URL);

    let response = client.get(url).send().await?;

    let mut response_body = response.json::<TimeShifterInfo>().await?;

    response_body.device_profile = Some(
        response_body
            ._profile
            .iter()
            .map(|s| parse_complex_str(&Commodity::Complex(s.to_string())).unwrap())
            .collect(),
    );

    response_body.electricity_consumption = Some(parse_complex_str(
        &response_body
            ._consumption
            .electricity
            .clone()
            .expect("Electricity commodity not found"),
    )?);

    Ok(response_body)
}

pub async fn get_jobs(house_id: u32, entity: TimeShifters) -> Result<Vec<Job>, ApiError> {
    let client = CLIENT.get_or_init(init);

    let entity_name = entity.get_device_name();
    let entity_id = format!("{entity_name}-House-{house_id}");

    let url = format!("{}/get/{entity_id}/jobs", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<Vec<Job>>().await?;

    Ok(response_body)
}

pub async fn schedule_job(
    house_id: u32,
    entity: TimeShifters,
    job: ScheduleJob,
) -> Result<Job, ApiError> {
    let client = CLIENT.get_or_init(init);

    let entity_name = entity.get_device_name();
    let entity_id = format!("{entity_name}-House-{house_id}");

    let url = format!("{}/callp/{entity_id}/scheduleJob", BASE_URL);

    let body = [job.delay, job.duration];
    let response = client.put(url).json(&body).send().await?;

    let response_body = response.json::<(bool, String)>().await?;

    let current_time = super::get_time().await;

    match response_body.0 {
        true => Ok(Job {
            start_time: current_time + job.delay,
            end_time: current_time + job.delay + job.duration,
        }),
        false => Err(ApiError::DemkitError(format!(
            "Failed to schedule job: {}",
            response_body.1
        ))),
    }
}

pub async fn cancel_job(house_id: u32, entity: TimeShifters, job_id: u32) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let entity_name = entity.get_device_name();
    let entity_id = format!("{entity_name}-House-{house_id}");

    let url = format!("{}/callp/{entity_id}/cancelJob", BASE_URL);

    let body = [job_id];
    let response = client.put(url).json(&body).send().await?;

    let (success, error_message) = response.json::<(bool, String)>().await?;

    match success {
        true => Ok(()),
        false => Err(ApiError::DemkitError(format!("Failed to cancel job: {}", error_message))),
    }
}

pub async fn force_shutdown(house_id: u32, entity: TimeShifters) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let entity_name = entity.get_device_name();
    let entity_id = format!("{entity_name}-House-{house_id}");

    let url = format!("{}/call/{entity_id}/forceShutdown", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<bool>().await?;

    match response_body {
        true => Ok(()),
        false => Err(ApiError::DemkitError(format!("Failed to force shutdown"))),
    }
}
