FROM rust:1.41

RUN apt update -yqq && apt install -yqq cmake g++

COPY ./ /poc-kafka-consumer
WORKDIR /poc-kafka-consumer

RUN cargo clean
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

CMD ["./target/release/poc-kafka-consumer", "-b", "kafka1:19092,kafka2:19092,kafka3:19092", "-g", "jon", "-t", "events"]
#CMD ["./target/release/poc-kafka-consumer"]

# OPTIONS:
#     -b, --brokers <brokers>      Broker list in kafka format [default: localhost:9092]
#     -g, --group-id <group-id>    Consumer group id [default: example_consumer_group_id]
#         --log-conf <log-conf>    Configure the logging format (example: 'rdkafka=trace')
#     -t, --topics <topics>...     Topic list

