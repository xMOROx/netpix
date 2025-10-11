mod asset;
mod client;
pub mod config;
mod constants;
mod handler;
mod sslkey;

use crate::sniffer::Sniffer;
use config::Config;
use std::collections::HashMap;
use warp::Filter;

use netpix_macros::{
    run_server, setup_clients, setup_packet_handlers, setup_routes, spawn_message_sender,
};

pub async fn run(sniffers: HashMap<String, Sniffer>, config: Config) {
    let clients = setup_clients!();

    let ssl_key_path = std::env::var("SSLKEYLOGFILE").ok();
     let _ssl_key_store = sslkey::start_ssl_key_watcher(ssl_key_path);

    let source_to_packets = setup_packet_handlers!((sniffers, clients, config));
    let sender_clients = clients.clone();

    let routes = setup_routes!((clients, source_to_packets));

    spawn_message_sender!((sender_clients, config.client_message_interval_ms,));

    run_server!((routes, config.addr));
}
