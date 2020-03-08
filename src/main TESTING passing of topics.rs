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

async fn consume_and_print(brokers: &str, group_id: &str, topics: Vec<&str>) {
// async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
        info!("brokers={}, group_id={}, topic={:?}", brokers, group_id, topics);
}

#[tokio::main]
async fn main() {
//    setup_logger(true, matches.value_of("log-conf"));
    let none: Option<&str> = None;
    setup_logger(true, none);

    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    // let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    // let brokers = matches.value_of("brokers").unwrap();
    // let group_id = matches.value_of("group-id").unwrap();

    let brokers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Missing environment variable KAFKA_BOOTSTRAP_SERVERS. Cannot continue without it.");
    let topic = env::var("KAFKA_TOPIC").expect("Missing environment variable KAFKA_TOPIC. Cannot continue without it.");
    let group_id = env::var("KAFKA_GROUP_ID").expect("Missing environment variable KAFKA_GROUP_ID. Cannot continue without it.");

    // let brokers = env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap();
    // let topic = env::var("KAFKA_TOPIC").unwrap();
    // let group_id = env::var("KAFKA_GROUP_ID").unwrap();


    let topics: Vec<&str> = vec!(&*topic);
//    v.push(&*topic);

//    println!("topics={:?}", v);
    // let topics = vec![&*topic];


    consume_and_print(&brokers, &group_id, topics).await
}