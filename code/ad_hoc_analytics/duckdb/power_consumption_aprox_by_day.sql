--Query to calculate aproximate electrical power consumption by day on each station
--      * Aproximation: Mean(Active Power)*h; how we are using full days h=24

with preprocess as (
    select
        year(event_ts) as event_year,
        month(event_ts) as event_month,
        day(event_ts) as event_day,
        *
    from lakehouse.default.op_metric
    where metric='Power.Active.Import'
)

select
    event_year,
    event_month,
    event_day,
    stationId,
    mean(value) as power_kw,
    mean(value)*24 as power_aprox_cons_kwh
from preprocess
group by
    event_year,
    event_month,
    event_day,
    stationId
;