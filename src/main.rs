#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

mod kafka_client;
mod kafka_config;

use kafka_config::KafkaProps;
use tokio;

use std::env;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        println!("expect 1 arg, but got {}", args.len());
        return;
    }

    let config: KafkaProps = serde_json::from_str(args[1].as_str())
        .map_err(|e| {
            println!("failed to parse json: {}", e);
        })
        .unwrap();
    println!("get config: {:?}", config);

    kafka_client::run_check(&config).await.unwrap();
}
