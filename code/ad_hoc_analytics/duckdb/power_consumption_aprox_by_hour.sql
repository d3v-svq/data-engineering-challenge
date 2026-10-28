--Query to calculate aproximate electrical power consumption by day on each station
--      * Aproximation: Mean(Active Power)*h; how we are using one hour interval h=1

with preprocess as (
    select
        year(event_ts) as event_year,
        month(event_ts) as event_month,
        day(event_ts) as event_day,
        hour(event_ts) as event_hour,
        *
    from lakehouse.default.op_metric
    where metric='Power.Active.Import'
)

select
    stationId,
    event_year,
    event_month,
    event_day,
    event_hour,
    mean(value) as power_kw,
    mean(value) as power_aprox_cons_kwh
from preprocess
group by
    event_year,
    event_month,
    event_day,
    event_hour,
    stationId
order by
    stationId,
    event_year,
    event_month,
    event_day,
    event_hour
;