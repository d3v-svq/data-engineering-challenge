use serde::{Deserialize, Serialize};

pub const AVRO_SCHEMA: &str = r#"
{
    "type": "record",
    "name": "OpMetric",
    "namespace": "com.ev.charging",
    "fields": [
        {"name": "event_ts", "type": {"type": "long", "logicalType": "timestamp-millis"}},
        {"name": "stationId", "type": "string"},
        {"name": "location", "type": "string"},
        {"name": "metric", "type": "string"},
        {"name": "value", "type": "float"},
        {"name": "unit", "type": "string"}
    ]
}
"#;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    pub value: String,
    #[serde(default)]
    pub measurand: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MeterValue {
    pub timestamp: String,
    #[serde(rename = "sampledValue", default)]
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Debug, Deserialize)]
pub struct MeterValuesPayload {
    #[serde(rename = "meterValue", default)]
    pub meter_value: Vec<MeterValue>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct OpMetric {
    pub event_ts: i64,
    #[serde(rename = "stationId")]
    pub station_id: String,
    pub location: String,
    pub metric: String,
    pub value: f32,
    pub unit: String,
}
