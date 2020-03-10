use clap::{App, Arg};
use futures::StreamExt;
use log::{info, warn};

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;

use crate::example_utils::setup_logger;
use std::option::Option;
mod example_utils;
use std::env;
use std::vec::Vec;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
// async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
        info!("brokers={}, group_id={}, topic={:?}", brokers, group_id, topics);
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
//    .subscribe(&topics.to_vec())
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let mut message_stream = consumer.start();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

#[tokio::main]
async fn main() {
    // let matches = App::new("consumer example")
    //     .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
    //     .about("Simple command line consumer")
    //     .arg(
    //         Arg::with_name("brokers")
    //             .short("b")
    //             .long("brokers")
    //             .help("Broker list in kafka format")
    //             .takes_value(true)
    //             .default_value("localhost:9092"),
    //     )
    //     .arg(
    //         Arg::with_name("group-id")
    //             .short("g")
    //             .long("group-id")
    //             .help("Consumer group id")
    //             .takes_value(true)
    //             .default_value("example_consumer_group_id"),
    //     )
    //     .arg(
    //         Arg::with_name("log-conf")
    //             .long("log-conf")
    //             .help("Configure the logging format (example: 'rdkafka=trace')")
    //             .takes_value(true),
    //     )
    //     .arg(
    //         Arg::with_name("topics")
    //             .short("t")
    //             .long("topics")
    //             .help("Topic list")
    //             .takes_value(true)
    //             .multiple(true)
    //             .required(true),
    //     )
    //     .get_matches();

    // setup_logger(true, matches.value_of("log-conf"));
    let none: Option<&str> = None;
    setup_logger(true, none);

    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    // let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    // let brokers = matches.value_of("brokers").unwrap();
    // let group_id = matches.value_of("group-id").unwrap();

    // println!("type of={:?}", matches.values_of("topics").unwrap());


    // println!("type of topics = {:?}", topics);

    // let jon = ["events"];
    // println!("type of jon = {:?}", jon);

    let brokers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Missing environment variable KAFKA_BOOTSTRAP_SERVERS. Cannot continue without it.");
    let topic = env::var("KAFKA_TOPIC").expect("Missing environment variable KAFKA_TOPIC. Cannot continue without it.");
    let group_id = env::var("KAFKA_GROUP_ID").expect("Missing environment variable KAFKA_GROUP_ID. Cannot continue without it.");

    // let brokers = env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap();
    // let topic_env = env::var("KAFKA_TOPIC").unwrap();
    // let group_id = env::var("KAFKA_GROUP_ID").unwrap();
    let topics = [&*topic];
    println!("type of topics = {:?}", topics);

//    let topics: Vec<&str> = vec!(&*topic);
//    v.push(&*topic);

//    println!("topics={:?}", v);
    // let topics = vec![&*topic];


    consume_and_print(&brokers, &group_id, &topics).await
}