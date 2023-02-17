use anyhow::Result;
use serde::{self, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Ohlc {
    pub symbol: String,
    pub timestamp: i64,
    #[serde(serialize_with = "f64_to_string")]
    pub open: f64,
    #[serde(serialize_with = "f64_to_string")]
    pub high: f64,
    #[serde(serialize_with = "f64_to_string")]
    pub low: f64,
    #[serde(serialize_with = "f64_to_string")]
    pub close: f64,
}

fn f64_to_string<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
