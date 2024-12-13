mod asset;
mod client;
mod handler;

use crate::sniffer::Sniffer;
pub use client::Client;
use std::collections::HashMap;
use std::net::SocketAddr;
use warp::Filter;

pub async fn run(sniffers: HashMap<String, Sniffer>, addr: SocketAddr) {
    let clients = client::new_clients();
    let source_to_packets = handler::setup_packet_handlers(sniffers, clients.clone()).await;

    let clients_filter = warp::any().map(move || clients.clone());
    let source_to_packets_filter = warp::any().map(move || source_to_packets.clone());
    let ws = warp::path(handler::WS_PATH)
        .and(warp::ws())
        .and(clients_filter)
        .and(source_to_packets_filter)
        .map(|ws: warp::ws::Ws, clients_cl, source_to_packets_cl| {
            ws.on_upgrade(move |socket| {
                client::handle_connection(socket, clients_cl, source_to_packets_cl)
            })
        });

    let index_html = warp::path::end().and_then(asset::serve_index);
    let other = warp::path::tail().and_then(asset::serve);

    let routes = ws.or(index_html).or(other);

    println!("Netpix running on http://{}/", addr);
    warp::serve(routes).try_bind(addr).await;
}
