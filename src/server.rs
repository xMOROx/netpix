mod asset;
mod client;
mod constants;
mod handler;

use crate::sniffer::Sniffer;
use std::collections::HashMap;
use std::net::SocketAddr;
use warp::Filter;

use netpix_macros::{
    run_server, setup_clients, setup_packet_handlers, setup_routes, spawn_message_sender,
};

pub async fn run(sniffers: HashMap<String, Sniffer>, addr: SocketAddr) {
    let clients = setup_clients!();
    let source_to_packets = setup_packet_handlers!((sniffers, clients));
    let sender_clients = clients.clone();

    let routes = setup_routes!((clients, source_to_packets));

    spawn_message_sender!((
        sender_clients,
        crate::server::constants::CLIENT_MESSAGE_INTERVAL_MS
    ));

    run_server!((routes, addr));
}
