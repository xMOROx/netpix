mod asset;
mod client;
pub mod config;
mod constants;
mod handler;

use crate::sniffer::Sniffer;
use config::Config;
use std::collections::HashMap;
use warp::Filter;

use netpix_macros::{
    run_server, setup_clients, setup_packet_handlers, setup_routes, spawn_message_sender,
};

pub async fn run(sniffers: HashMap<String, Sniffer>, config: Config) {
    let clients = setup_clients!();
    let source_to_packets = setup_packet_handlers!((sniffers, clients, config));
    let sender_clients = clients.clone();

    let routes = setup_routes!((clients, source_to_packets, config));

    spawn_message_sender!((sender_clients, config.client_message_interval_ms, config.message_batch_size));

    run_server!((routes, config.addr));
}
