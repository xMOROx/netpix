mod asset;
mod client;
mod constants;
mod handler;

use crate::sniffer::Sniffer;
use constants::{CLIENT_MESSAGE_INTERVAL_MS, WEBSOCKET_PATH};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::interval;
use warp::Filter;

pub async fn run(sniffers: HashMap<String, Sniffer>, addr: SocketAddr) {
    let clients = client::new_clients();
    let source_to_packets = handler::setup_packet_handlers(sniffers, clients.clone()).await;

    let clients_clone = clients.clone();
    let clients_filter = warp::any().map(move || clients_clone.clone());
    let source_to_packets_filter = warp::any().map(move || source_to_packets.clone());
    let ws = warp::path(WEBSOCKET_PATH)
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

    let clients_for_sender = clients.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(CLIENT_MESSAGE_INTERVAL_MS));
        loop {
            ticker.tick().await;
            let mut clients = clients_for_sender.write().await;

            for client in clients.values_mut() {
                if let Some(msg) = client.queue.pop_front() {
                    client.sender.send(msg).ok();
                }
            }
        }
    });

    println!("Netpix running on http://{}/", addr);
    warp::serve(routes).try_bind(addr).await;
}
