use crate::kafka_config::KafkaProps;
use futures_async_stream::for_await;
use futures_util::StreamExt;
use rdkafka::consumer::{Consumer, DefaultConsumerContext, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::{ClientConfig, TopicPartitionList};
use rdkafka::Message;
pub async fn run_check(config: &KafkaProps) -> Result<(), String> {
    let mut client_config = ClientConfig::new();

        client_config.set("bootstrap.servers", config.brokers.as_str())
        .set(
            "group.id",
            config
                .group_id
                .as_ref()
                .unwrap_or(&"rw-consumer-1-1".to_string())
                .as_str(),
        )
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("isolation.level", "read_committed")
        .set("auto.offset.reset", "earliest");
    let consumer: StreamConsumer<DefaultConsumerContext> =
        client_config.create().await.map_err(|e| e.to_string())?;
    println!("consumer created");

    println!("performing list partitions");
    let now = chrono::Utc::now();
    let metadata = consumer
        .fetch_metadata(Some(config.topic.as_str()), config.sync_call_timeout)
        .await
        .map_err(|e| e.to_string())?;
    let elapsed = chrono::Utc::now().signed_duration_since(now);
    let topic_meta = match metadata.topics() {
        [meta] => meta,
        _ => return Err(format!("topic {} not found", config.topic)),
    };

    if topic_meta.partitions().is_empty() {
        return Err(format!("topic {} has 0 partitions", config.topic));
    }
    println!(
        "topic {} has {} partitions, request takes {}ms",
        config.topic,
        topic_meta.partitions().len(),
        elapsed.num_milliseconds()
    );

    println!("testing consume topic {}", config.topic);
    let mut tp_list = TopicPartitionList::new();
    for partition in topic_meta.partitions() {
        tp_list.add_partition(&config.topic, partition.id());
    }
    consumer.assign(&tp_list).map_err(|e| e.to_string())?;

    #[for_await]
    for msgs in consumer.stream().ready_chunks(5) {
        let msgs: Vec<_> = msgs
            .into_iter()
            .collect::<std::result::Result<_, KafkaError>>().map_err(|e| e.to_string())?;
        for msg in &msgs {
            println!(
                "topic: {}, partition: {}, offset: {}, key len: {}, payload len: {}",
                msg.topic(),
                msg.partition(),
                msg.offset(),
                msg.key().map(|k| k.len()).unwrap_or(0),
                msg.payload().map(|p| p.len()).unwrap_or(0),
            );
        }
        break;
    }

    println!("test success");
    Ok(())
}
