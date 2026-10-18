use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// File with input data to parse
    #[arg(long)]
    pub input_file: String,
    /// Kafka broker url in format -> <host|ip>:<port>
    #[arg(long)]
    pub kafka_broker_url: String,
    /// Kafka topic name
    #[arg(long)]
    pub kafka_topic: String,
    /// Kafka schema registry url in format -> http://<host|ip>:<port>
    #[arg(long)]
    pub kafka_schema_registry_url: String,
}
