// Query data filtered by station and location dropdown variables
data = from(bucket: "ocpp_data")
  |> range(start: v.timeRangeStart, stop: v.timeRangeStop)
  |> filter(fn: (r) => r._measurement == "ocpp_operational_metrics")
  |> filter(fn: (r) => r._field == "value")
  |> filter(fn: (r) => r.location == v.location)
  |> filter(fn: (r) => r.stationId == v.station)
  |> group(columns: ["metric", "unit"])

// Calculate aggregates independently
min_val = data |> min() |> set(key: "_stat", value: "min")
max_val = data |> max() |> set(key: "_stat", value: "max")
mean_val = data |> mean() |> set(key: "_stat", value: "mean")

// Combine and pivot into a clean tabular structure
union(tables: [min_val, max_val, mean_val])
  |> pivot(rowKey: ["metric", "unit"], columnKey: ["_stat"], valueColumn: "_value")
  |> keep(columns: ["metric", "unit", "min", "max", "mean"])