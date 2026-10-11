use crate::models::{MeterValuesPayload, OpMetric};
use chrono::DateTime;
use std::fs::File;
use std::io::{BufRead, BufReader};


/// Parses a raw log line into a stream of flattened OpMetrics
pub fn parse_line_to_metrics(line: &str) -> Option<impl Iterator<Item = OpMetric> + '_> {
    let (station_id_raw, json_raw) = line.split_once(':')?;
    let station_id = station_id_raw.trim().to_string();

    let parsed_array: Vec<serde_json::Value> = serde_json::from_str(json_raw.trim()).ok()?;

    if parsed_array.len() < 4 || parsed_array[0] != 2 || parsed_array[2] != "MeterValues" {
        return None;
    }

    let payload_val = parsed_array.get(3)?;
    let payload: MeterValuesPayload = serde_json::from_value(payload_val.clone()).ok()?;

    let metrics = payload.meter_value.into_iter().flat_map(move |mv| {
        // Parse ISO8601 string to epoch milliseconds
        let ts_millis = DateTime::parse_from_rfc3339(&mv.timestamp)
            .map(|dt| dt.timestamp_millis())
            .unwrap_or(0);
        let sid = station_id.clone();

        mv.sampled_value.into_iter().filter_map(move |sv| {
            // Parse value to float, skip metric if parsing fails
            let parsed_value = sv.value.parse::<f32>().ok()?;

            Some(OpMetric {
                event_ts: ts_millis,
                station_id: sid.clone(),
                location: sv.location.unwrap_or_else(|| "UnknowLocation".to_string()),
                metric: sv.measurand.unwrap_or_else(|| "UnknowMetric".to_string()),
                value: parsed_value,
                unit: sv.unit.unwrap_or_else(|| "UnknowUnit".to_string()),
            })
        })
    });

    Some(metrics)
}

/// Reads a file line-by-line and extracts metrics lazily
pub fn extract_metrics_from_file(file_path: &str) -> anyhow::Result<impl Iterator<Item = OpMetric>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let metrics = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| parse_line_to_metrics(&line).map(|iter| iter.collect::<Vec<_>>()))
        .flatten();

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meter_values() {
        let input = r#"charger1 :
            [
                2,
                "6143a7d5",
                "MeterValues",
                {
                    "connectorId":1,
                    "transactionId":1,
                    "meterValue":[
                        {
                            "timestamp":"2025-08-23T00:59:02.720Z",
                            "sampledValue":[
                                {
                                    "value":"24.8",
                                    "context":"Sample.Periodic",
                                    "format":"Raw",
                                    "measurand":"Current.Import",
                                    "location":"Outlet",
                                    "unit":"A"
                                },
                                {
                                    "value":"19334.1709",
                                    "context":"Sample.Periodic",
                                    "format":"Raw",
                                    "measurand":"Energy.Active.Import.Register",
                                    "location":"Outlet",
                                    "unit":"kWh"
                                }
                            ]
                        }
                    ]
                }
            ]"#;

        let metrics: Vec<OpMetric> = parse_line_to_metrics(input).unwrap().collect();

        assert_eq!(metrics.len(), 2);
        assert_eq!(metrics[0], OpMetric {
            event_ts: 1755910742720, // 2025-08-23T00:59:02.720Z in millis
            station_id: "charger1".to_string(),
            location: "Outlet".to_string(),
            metric: "Current.Import".to_string(),
            value: 24.8,
            unit: "A".to_string(),
        });
        assert_eq!(metrics[1], OpMetric {
            event_ts: 1755910742720, // 2025-08-23T00:59:02.720Z in millis
            station_id: "charger1".to_string(),
            location: "Outlet".to_string(),
            metric: "Energy.Active.Import.Register".to_string(),
            value: 19334.1709,
            unit: "kWh".to_string(),
        });
    }

    #[test]
    fn test_ignore_heartbeat() {
        let input = r#"charger1 : [2,"a408cf3e","Heartbeat",{ }]"#;
        assert!(parse_line_to_metrics(input).is_none());
    }

    #[test]
    fn test_invalid_float_value() {
        let input = r#"charger1 : [2,"123","MeterValues",{"meterValue":[{"timestamp":"2025-08-23T00:59:02.720Z","sampledValue":[{"value":"invalid","measurand":"Current.Import"}]}]}]"#;
        let metrics: Vec<OpMetric> = parse_line_to_metrics(input).unwrap().collect();
        assert_eq!(metrics.len(), 0); // Should be filtered out due to f32 parsing failure
    }
}