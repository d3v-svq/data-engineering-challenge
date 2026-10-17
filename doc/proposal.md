This documentation shows how the challenge is approximated.

# Main goals
Build a data platform which: 
* offers a unique source of truth
* generates business value as soon as data is generated
* facilitates operational status analysis (both streaming and historical)
* improves scalability and flexibility (ad-hoc analysis)

## High level implementation
| Goal                                              | Abstraction                                        | Tools which can be used                                   |
|---------------------------------------------------|----------------------------------------------------|-----------------------------------------------------------|
| Unique source of truth (streaming and historical) | Lakestream (~message/events system plus lakehouse) | Open Lakestream, Apache Fluss or Apache Kafka + Lakehouse |
| Streaming analytics                               | Streaming engine and/or Timeseries database        | Apache Flink and/or Influxdb + grafana                    |
| Batch analytics                                   | Batch engine                                       | Apache Flink/Apache Spark                                 |
| Ad-hoc analysis                                   | SQL query engine                                   | Trino/Athena/DuckDB                                       |

## High level diagram

# Prototype
Taking into account the main goals and the high level implementation:

### What this prototype brings
* A little simple-dashboard with some stats about operational metric in real time.
* A little example about doing ad-hoc analysis using historical data.
* An end-to-end data-platform which offers a proof of concept for growing and scale with small changes.
* A rust kafka-producer trying to do functional programming (I try my best, I promise :D) with some unit tests. 
  * Disclaimer: It's one of my first rust development so don't take it too seriously.
* How to guide to run it locally.
* This small and pretty doc.

### What this propotype DOESN'T bring :(
* Production ready version of services used (kafka, influxdb, object storage, etc.).
* Cross services definition or implementation (data governance, automatism, observability, etc.).
* Historical analytics using batch engine.
* Event time processing because entries in input data files has timestamps from past.
* Only operational metrics are parsed.


## Architecture
### Diagram
#### Diagram elements
* **Producer**: in charge of parse the input data files and publish data to lakestream in AVRO format.
* **Lakestream**: open stream storage system which mixes kafka and iceberg features for offering a unique source of truth.
  * kafka topic for storing  real-time operational metrics events in avro format.
  * schema registry activated to enable data contracts between producers and consumers.
  * iceberg table with the same data schema of kafka topic
* **Telegraf**: read data from kafka topic and publish it to influxdb table.
* **Influxdb**: time-series databases where streaming analytics will be performed.
* **DuckDB**: SQL query engine to perform queries in iceberg tables for ad-hoc analysis.

## Data Models
### Kafka
#### op_metric
Data model defined is as follows (using kafka nomenclature):
* topic: op_metric
* format: avro
  * schema: 
  ```
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
  ```


### Iceberg
### Influxdb
#### ocpp_operational_metrics
Data model defined is as follows (using influxdb nomenclature):
* **measurement**: ocpp_operational_metrics
* **tags** (dimensions): 
    * stationId
    * location
    * metric 
    * unit
* **fields** (metrics):
  * value
