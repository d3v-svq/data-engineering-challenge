In this document we are going to guide step by step how to run the demo for the prototype.

All services used are placed in infra directory for its deployment, so:
1. Change to that directory
````
cd infra
````
2. Run docker compose using e2e profile
````
docker compose --profile e2e up -d
````
In case you only want to deploy the "streaming part" you can run it with the streaming-analytics profile:
````
docker compose --profile streaming-analytics up -d
````
In case you place all services in a different machine from data producer (how is mine) you must advertise the kafka host to enable connectivity:
````
KAFKA_ADVERTISED_LISTENERS_HOST=<HOST_IP|HOSTNAME> docker compose --profile streaming-analytics up -d
# you can modify the value in the .env file too, in case you prefer it
````

Docker compose deploy and run:
* **Kafka broker**
* **Schema registry**
* **Catalogs**: streams (oxia) and iceberg tables (polaris)
* **MinIO**
* **Telegraf**
* **InfluxDB**
* Also, some **operations** are **performed** to have all ready for ingesting data and creating dashboards:
  * **creation of the topic** `op_metric`
  * **creation of entry in schema registry for avro schema** used in `op_metric` topic

3. Start the ingestion phase using the kafka producer, placed in the `code/ocpp_kafka_producer` directory execute the rust program:

    In case you run the producer in the same machine as all services (deployed by docker compose) you can use:
    ````
    cd code/ocpp_kafka_producer
    cargo run -- --input-file "../../data/ocpp-data-many-chargers.txt" --kafka-broker-url localhost:29092 --kafka-topic op_metric --kafka-schema-registry-url localhost:18081
    ````
    In case you run the producer in a different machine where services are running, you must execute:
    ````
    cd code/ocpp_kafka_producer
    # REMEMBER TO SUBSTITUTE THE PLACEHOLDER <ANOTHER_MACHINE_IP> WITH THE CORRESPONDIG VALUE
    cargo run -- --input-file "../../data/ocpp-data-many-chargers.txt" --kafka-broker-url <ANOTHER_MACHINE_IP>:29092 --kafka-topic op_metric --kafka-schema-registry-url <ANOTHER_MACHINE_IP>:18081
    ````

4. Now you can use the influxDB UI to perform some analytics in near-real time or duckdb for ad-hoc analysis

   * Influxdb:
     * In a web browser you can access to the url http://localhost:8086
     * user: admin
     * password: AdminPassword123!
   * Duckdb:
     * Run the command `docker container start -ai hivemind-ev-duckdb-1` 
