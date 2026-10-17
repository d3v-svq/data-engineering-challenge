from(bucket: "ocpp_data")
  |> range(start: v.timeRangeStart, stop: v.timeRangeStop)
  |> filter(fn: (r) => r._measurement == "ocpp_operational_metrics")
  |> filter(fn: (r) => r.metric == v.metric)
  |> filter(fn: (r) => r._field == "value")
  // Grouping by stationId creates a distinct series line per station
  |> group(columns: ["stationId"])
  |> aggregateWindow(every: v.windowPeriod, fn: mean, createEmpty: false)
  |> yield(name: "mean")