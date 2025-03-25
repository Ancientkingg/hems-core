use std::fmt;

use num_complex::Complex;
use serde::{de::{self, SeqAccess, Visitor}, Deserialize, Deserializer};

use super::{init, parse_complex_str, ApiError, Commodities, BASE_URL, CLIENT};

#[derive(Debug)]
pub struct Job {
    pub id: u32,
    pub start_time: u64,
    pub end_time: u64,
}

impl<'de> Deserialize<'de> for Job {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JobVisitor;

        impl<'de> Visitor<'de> for JobVisitor {
            type Value = Job;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array with an integer ID and an object containing startTime and endTime")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let id: u32 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let mut start_time: Option<u64> = None;
                let mut end_time: Option<u64> = None;

                if let Some(map) = seq.next_element::<serde_json::Value>()? {
                    if let Some(obj) = map.as_object() {
                        start_time = obj.get("startTime").and_then(|v| v.as_u64());
                        end_time = obj.get("endTime").and_then(|v| v.as_u64());
                    }
                }

                let start_time = start_time.ok_or_else(|| de::Error::missing_field("startTime"))?;
                let end_time = end_time.ok_or_else(|| de::Error::missing_field("endTime"))?;

                Ok(Job { id, start_time, end_time })
            }
        }

        deserializer.deserialize_seq(JobVisitor)
    }
}

pub enum TimeShifters {
    Dishwasher,
    WashingMachine,
}

impl TimeShifters {
    pub fn get_device_name(&self) -> &str {
        match self {
            TimeShifters::Dishwasher => "Dishwasher",
            TimeShifters::WashingMachine => "WashingMachine",
        }
    }
}


pub async fn get_jobs(house_id: u32, entity: TimeShifters) -> Result<Vec<Job>, ApiError> {
    let client = CLIENT.get_or_init(init);

    let entity_name = entity.get_device_name();
    let entity_id = format!("{entity_name}-House-{house_id}");

    let url = format!(
        "{}/get/{entity_id}/jobs",
        BASE_URL
    );

    let response = client.get(url).send().await?;

    let response_body = response.json::<Vec<Job>>().await?;

    Ok(response_body)
}

pub async fn get_job_by_id(house_id: u32, entity: TimeShifters, job_id: u32) -> Result<Job, ApiError> {
    let jobs = get_jobs(house_id, entity).await?;

    todo!("")
}

pub async fn schedule_job(_house_id: u32, _entity_id: u32) -> Result<Job, ApiError> {
    todo!("")
}

pub async fn cancel_job(_house_id: u32, _entity_id: u32, _job_id: u32) -> Result<(), ApiError> {
    todo!("")
}