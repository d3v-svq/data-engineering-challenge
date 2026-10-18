#!/bin/sh
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -eu

: "${KAFKA_TOPIC:?DEMO_TOPIC is required}"
: "${SCHEMA_REGISTRY_URL:?SCHEMA_REGISTRY_URL is required}"
subject="${KAFKA_TOPIC}-value"


# AVRO SCHEMA to register
value_schema='{
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
}'

# The registry API takes the schema as a JSON string, so escape the quotes.
schema_as_json_string="$(printf '%s' "$value_schema" | tr -d '\n' | sed 's/"/\\"/g')"

# Registering the same schema twice returns the same id, so this is idempotent.
# The registry answers its health check a little before it has finished the
# primary election that registrations need, hence the retry.
attempt=1
while :; do
  if response="$(curl -sS --fail-with-body -X POST \
      -H 'Content-Type: application/vnd.schemaregistry.v1+json' \
      --data "{\"schema\": \"${schema_as_json_string}\"}" \
      "${SCHEMA_REGISTRY_URL}/subjects/${subject}/versions")"; then
    break
  fi
  if [ "$attempt" -ge 30 ]; then
    echo "Schema registration failed: ${response}" >&2
    exit 1
  fi
  echo "Schema registry is not accepting registrations yet, retrying... (${attempt}/30)"
  attempt=$((attempt + 1))
  sleep 2
done
schema_id="$(printf '%s' "$response" \
  | sed -n 's/.*"id"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p')"
if [ -z "$schema_id" ]; then
  echo "Could not parse a schema id from: ${response}" >&2
  exit 1
fi
echo "Registered schema id ${schema_id} as subject ${subject}."