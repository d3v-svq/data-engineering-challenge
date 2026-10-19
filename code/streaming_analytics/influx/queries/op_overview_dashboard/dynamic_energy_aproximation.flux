// 1. Calculate duration in hours between active zoom/range bounds
stopNs = float(v: int(v: v.timeRangeStop))
startNs = stopNs - float(v: int(v: v.timeRangeStart))
durationHours = (stopNs - startNs) / float(v: -3600000000000)

// 2. Average Power multiplied by duration in hours
from(bucket: "ocpp_data")
  |> range(start: v.timeRangeStart, stop: v.timeRangeStop)
  |> filter(fn: (r) => r._measurement == "ocpp_operational_metrics")
  |> filter(fn: (r) => r.metric == "Power.Active.Import")
  |> filter(fn: (r) => r._field == "value")
  |> filter(fn: (r) => r.location == v.location)
  |> filter(fn: (r) => r.stationId == v.station)
  |> group()
  |> mean()
  |> map(fn: (r) => ({
      _time: v.timeRangeStop,
      _field: "approx_energy",
      _value: r._value * durationHours
    }))