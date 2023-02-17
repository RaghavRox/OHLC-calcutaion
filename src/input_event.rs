use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct InputEvent {
    e: String,
    u: i64,
    pub(crate) s: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub(crate) b: f64,
    #[serde(deserialize_with = "string_to_f32")]
    B: f32,
    #[serde(deserialize_with = "string_to_f64")]
    pub(crate) a: f64,
    #[serde(deserialize_with = "string_to_f32")]
    A: f32,
    pub(crate) T: i64,
    E: i64,
}

impl FromStr for InputEvent {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

fn string_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f32>().map_err(serde::de::Error::custom)
}
