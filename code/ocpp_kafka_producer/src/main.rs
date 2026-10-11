mod models;
mod parser;

use anyhow::{Context, Result};
use apache_avro::schema::Schema;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use schema_registry_converter::async_impl::avro::AvroEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

use crate::models::AVRO_SCHEMA;
use crate::parser::extract_metrics_from_file;

#[tokio::main]
async fn main() -> Result<()> {
    let input_file = "../../data/ocpp-data-many-chargers-small-sample.txt";
    let brokers = "localhost:29092";
    let schema_registry_url = "http://localhost:18081";
    let topic = "op_metric";

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .context("Failed to create Kafka producer")?;

    let sr_settings = SrSettings::new(schema_registry_url.to_string());
    let encoder = AvroEncoder::new(sr_settings);

    let _schema = Schema::parse_str(AVRO_SCHEMA).context("Failed to parse Avro Schema")?;
    let subject_name_strategy = SubjectNameStrategy::TopicNameStrategy(topic.to_string(), false);

    let metrics_iter = extract_metrics_from_file(input_file)?;

    for metric in metrics_iter {
        let value_payload = serde_json::to_value(&metric)?;

        let encoded_payload = encoder
            .encode_struct(value_payload, &subject_name_strategy)
            .await
            .context("Failed to encode payload via Schema Registry")?;

        let record = FutureRecord::to(topic)
            .key(&metric.station_id)
            .payload(&encoded_payload);

        match producer.send(record, tokio::time::Duration::from_secs(0)).await {
            Ok((partition, offset)) => {
                println!(
                    "Published {} metric for station {} to partition {} at offset {}",
                    metric.metric, metric.station_id, partition, offset
                );
            }
            Err((e, _)) => eprintln!("Failed to publish message: {:?}", e),
        }
    }

    Ok(())
}