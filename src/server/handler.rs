use super::{client::Clients, config::Config};
use crate::sniffer::Sniffer;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryFutureExt,
};
use log::{error, info, warn};
use netpix_common::{PacketsStats, Request, Response, RtpStreamKey, Sdp, Source};
use ringbuf::{
    traits::{Consumer, Observer, RingBuffer},
    HeapRb,
};
use std::collections::HashSet;
use std::time::SystemTime;
use std::{collections::HashMap, io::Write, sync::Arc};
use tokio::sync::{mpsc, mpsc::UnboundedSender, RwLock};
use warp::ws::{Message, WebSocket};

pub type PacketRingBuffer = HeapRb<Response>;
pub type Packets = Arc<RwLock<PacketRingBuffer>>;
pub type PacketsMap = Arc<HashMap<Source, (Packets, mpsc::Sender<()>)>>;

/// Per-source list of subscribed client IDs for efficient broadcasting
pub type SourceSubscriptions = Arc<RwLock<HashMap<Source, HashSet<usize>>>>;

pub async fn setup_packet_handlers(
    sniffers: HashMap<String, Sniffer>,
    clients: Clients,
    config: Config,
) -> PacketsMap {
    let mut source_to_packets = HashMap::new();

    for (_file, sniffer) in sniffers {
        let packets = Arc::new(RwLock::new(HeapRb::new(config.packet_buffer_size)));
        let (cancel_tx, cancel_rx) = mpsc::channel(1);
        source_to_packets.insert(sniffer.source.clone(), (packets.clone(), cancel_tx));

        let cloned_clients = clients.clone();
        tokio::task::spawn(async move {
            sniff(sniffer, packets, cloned_clients, config, cancel_rx).await;
        });
    }

    Arc::new(source_to_packets)
}

pub async fn send_pcap_filenames(
    client_id: &usize,
    ws_tx: &mut SplitSink<WebSocket, Message>,
    source_to_packets: &Arc<HashMap<Source, (Packets, mpsc::Sender<()>)>>,
) {
    let sources = source_to_packets.keys().cloned().collect();
    let response = Response::Sources(sources);

    let Ok(encoded) = response.encode() else {
        error!("Failed to encode packet, client_id: {}", client_id);
        return;
    };

    let msg = Message::binary(encoded);
    ws_tx
        .send(msg)
        .unwrap_or_else(|e| {
            error!("WebSocket send error: {}, client_id: {}", e, client_id);
        })
        .await;
}

async fn send_stats(clients: &Clients, discharged: usize, overwritten: usize) {
    let stats = PacketsStats {
        discharged,
        overwritten,
    };
    let response = Response::PacketsStats(stats);
    
    // Encode once and reuse for all clients
    let Ok(encoded) = response.encode() else {
        error!("Failed to encode stats");
        return;
    };
    let msg = Message::binary(encoded);
    
    for (_, client) in clients.write().await.iter_mut() {
        client.try_queue_message(msg.clone());
    }
}

async fn discharge_old_packets(packets: &mut PacketRingBuffer, max_packets_age: u64) -> usize {
    let now = SystemTime::now();
    let mut discharged_count = 0;
    while let Some(packet) = packets.try_peek() {
        if let Response::Packet(p) = packet {
            match now.duration_since(p.creation_time) {
                Ok(age) if age.as_secs() <= max_packets_age => break,
                _ => {
                    packets.try_pop();
                    discharged_count += 1;
                }
            }
        } else {
            break;
        }
    }

    discharged_count
}

async fn sniff(
    mut sniffer: Sniffer,
    packets: Packets,
    clients: Clients,
    config: Config,
    mut cancel_rx: mpsc::Receiver<()>,
) {
    let mut overwritten_count = 0;
    let mut total_discharged_count = 0;
    let mut last_stats_time = SystemTime::now();

    loop {
        tokio::select! {
            _ = cancel_rx.recv() => {
                // Source changed, stop processing
                break;
            }
            result = sniffer.next_packet() => {
                match result {
                    Some(Ok(mut pack)) => {
                        pack.guess_payload();
                        let response = Response::Packet(pack);

                        let Ok(encoded) = response.encode() else {
                            error!("Sniffer: failed to encode packet");
                            continue;
                        };
                        let msg = Message::binary(encoded);

                        // Use try_queue_message with backpressure handling
                        let mut clients_guard = clients.write().await;
                        for (_, client) in clients_guard.iter_mut() {
                            if let Some(src) = &client.source {
                                if *src == sniffer.source {
                                    client.try_queue_message(msg.clone());
                                }
                            }
                        }
                        drop(clients_guard);

                        let mut packets = packets.write().await;

                        let discharged = discharge_old_packets(&mut packets, config.max_packets_age).await;
                        total_discharged_count += discharged;

                        if packets.is_full() {
                            overwritten_count += 1;
                            warn!("Packet buffer full, discarding oldest packet");
                        }
                        packets.push_overwrite(response);

                        if let Ok(elapsed) = last_stats_time.elapsed() {
                            if elapsed.as_secs() >= 5 {
                                send_stats(&clients, total_discharged_count, overwritten_count).await;
                                last_stats_time = SystemTime::now();
                            }
                        }
                    }
                    Some(Err(err)) => info!("Error when capturing a packet: {:?}", err),
                    None => break,
                }
            }
        }
    }
}

async fn send_batch(packets: Vec<Response>, ws_tx: &UnboundedSender<Message>, client_id: usize) {
    let encoded =
        bincode::encode_to_vec(&packets, bincode::config::standard()).unwrap_or_else(|e| {
            error!("Failed to encode packet batch: {}", e);
            Vec::new()
        });

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&encoded).unwrap_or_else(|e| {
        error!("Failed to compress packet batch: {}", e);
    });

    let compressed = encoder.finish().unwrap_or_else(|e| {
        error!("Failed to finish compression: {}", e);
        Vec::new()
    });

    let msg = Message::binary(compressed);
    ws_tx.send(msg).unwrap_or_else(|e| {
        error!(
            "WebSocket batch send error: {}, client_id: {}",
            e, client_id
        );
    });
}

async fn send_all_packets(client_id: usize, packets: &Packets, clients: &Clients) {
    let packets_read = packets.read().await;
    let mut wr_clients = clients.write().await;
    let client = match wr_clients.get_mut(&client_id) {
        Some(client) => client,
        None => return, // The client might have disconnected
    };

    for packet in packets_read.iter() {
        let Ok(encoded) = packet.encode() else {
            error!("Failed to encode packet for client_id: {}", client_id);
            continue;
        };
        let msg = Message::binary(encoded);

        client.try_queue_message(msg);
    }
}

async fn parse_sdp(
    client_id: usize,
    clients: &Clients,
    cur_source: &Source,
    stream_key: RtpStreamKey,
    raw_sdp: String,
) {
    let Some(sdp) = Sdp::build(raw_sdp) else {
        warn!(
            "Received invalid SDP for {:?}: {:?}",
            cur_source, stream_key
        );
        return;
    };

    let Ok(encoded) = Response::Sdp(stream_key, sdp).encode() else {
        error!("Failed to encode sdp, client_id: {}", client_id);
        return;
    };

    let msg = Message::binary(encoded);

    let clients_guard = clients.read().await;
    for (_, client) in clients_guard.iter() {
        if let Some(ref source) = client.source {
            if *source == *cur_source {
                if let Err(e) = client.sender.send(msg.clone()) {
                    error!("Sniffer: error while sending sdp: {}", e);
                }
            }
        }
    }
}

async fn handle_source_change(
    client_id: usize,
    new_source: Source,
    clients: &Clients,
    packets: &PacketsMap,
) -> bool {
    if let Some((new_packets, _)) = packets.get(&new_source) {
        let mut wr_clients = clients.write().await;
        let client = wr_clients.get_mut(&client_id).unwrap();

        if let Some(old_source) = &client.source {
            if let Some((_, cancel_tx)) = packets.get(old_source) {
                let _ = cancel_tx.send(()).await;
            }
        }

        client.queue.clear();

        client.source = Some(new_source.clone());
        drop(wr_clients);

        send_all_packets(client_id, new_packets, clients).await;
        true
    } else {
        false
    }
}

pub async fn handle_messages(
    client_id: usize,
    mut ws_rx: SplitStream<WebSocket>,
    clients: &Clients,
    packets: &PacketsMap,
) {
    let rd_clients = clients.read().await;
    let client = rd_clients.get(&client_id).unwrap();
    let mut source = client.source.clone();
    drop(rd_clients);

    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                info!("Received message: {:?}, client_id: {}", msg, client_id);
                if !msg.is_binary() {
                    continue;
                }

                let msg_bytes = msg.into_bytes();
                let Ok(req) = Request::decode(&msg_bytes) else {
                    error!("Failed to decode request message, client_id: {}", client_id);
                    continue;
                };

                match req {
                    (Request::FetchAll, _) => {
                        if let Some(ref cur_source) = source {
                            if let Some((packets, _)) = packets.get(cur_source) {
                                send_all_packets(client_id, packets, clients).await;
                            } else {
                                warn!(
                                    "No packets found for source: {:?}, client_id: {}",
                                    cur_source, client_id
                                );
                            }
                        }
                    }
                    (Request::ChangeSource(new_source), _) => {
                        if handle_source_change(client_id, new_source.clone(), clients, packets)
                            .await
                        {
                            source = Some(new_source);
                        } else {
                            warn!(
                                "Attempted to change to unknown source: {:?}, client_id: {}",
                                new_source, client_id
                            );
                        }
                    }

                    (Request::ParseSdp(stream_key, sdp), _) => {
                        if let Some(cur_source) = &source {
                            parse_sdp(client_id, clients, cur_source, stream_key, sdp).await;
                        } else {
                            warn!("Received ParseSdp request without a selected source, client_id: {}", client_id);
                        }
                    }

                    (Request::PacketsStats(stats), _) => {
                        let response = Response::PacketsStats(stats);
                        if let Ok(encoded) = response.encode() {
                            let msg = Message::binary(encoded);
                            let mut wr_clients = clients.write().await;
                            for (_, client) in wr_clients.iter_mut() {
                                client.try_queue_message(msg.clone());
                            }
                        }
                    }

                    _ => {
                        warn!("Unhandled request: {:?}, client_id: {}", req, client_id);
                    }
                }
            }
            Err(e) => {
                error!("WebSocket error: {}, client_id: {}", e, client_id);
                break;
            }
        }
    }
}
