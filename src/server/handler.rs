use super::client::Clients;
use crate::sniffer::Sniffer;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryFutureExt,
};
use log::{error, info, warn};
use netpix_common::{Request, Response, RtpStreamKey, Sdp, Source, PACKET_MAX_AGE_SECS};
use ringbuf::{
    traits::{Consumer, Observer, RingBuffer},
    HeapRb,
};
use std::time::SystemTime;
use std::{collections::HashMap, io::Write, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use warp::ws::{Message, WebSocket};

use super::constants::*;

pub type PacketRingBuffer = HeapRb<Response>;
pub type Packets = Arc<RwLock<PacketRingBuffer>>;
pub type PacketsMap = Arc<HashMap<Source, Packets>>;

pub async fn setup_packet_handlers(
    sniffers: HashMap<String, Sniffer>,
    clients: Clients,
) -> PacketsMap {
    let mut source_to_packets = HashMap::new();

    for (_file, sniffer) in sniffers {
        let packets = Arc::new(RwLock::new(HeapRb::new(PACKET_BUFFER_SIZE)));
        source_to_packets.insert(sniffer.source.clone(), packets.clone());

        let cloned_clients = clients.clone();
        tokio::task::spawn(async move {
            sniff(sniffer, packets, cloned_clients).await;
        });
    }

    Arc::new(source_to_packets)
}

pub async fn send_pcap_filenames(
    client_id: &usize,
    ws_tx: &mut SplitSink<WebSocket, Message>,
    source_to_packets: &Arc<HashMap<Source, Packets>>,
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

async fn discharge_old_packets(packets: &mut PacketRingBuffer) {
    let now = SystemTime::now();
    while let Some(packet) = packets.try_peek() {
        if let Response::Packet(p) = packet {
            match now.duration_since(p.creation_time) {
                Ok(age) if age.as_secs() <= PACKET_MAX_AGE_SECS => break,
                _ => {
                    packets.try_pop();
                }
            }
        } else {
            break;
        }
    }
}

async fn sniff(mut sniffer: Sniffer, packets: Packets, clients: Clients) {
    while let Some(result) = sniffer.next_packet().await {
        match result {
            Ok(mut pack) => {
                pack.guess_payload();
                let response = Response::Packet(pack);

                let Ok(encoded) = response.encode() else {
                    error!("Sniffer: failed to encode packet");
                    continue;
                };
                let msg = Message::binary(encoded);

                for (_, client) in clients.write().await.iter_mut() {
                    if let Some(src) = &client.source {
                        if *src == sniffer.source {
                            client.queue.push_back(msg.clone());
                        }
                    }
                }

                let mut packets = packets.write().await;

                discharge_old_packets(&mut packets).await;

                if packets.is_full() {
                    warn!("Packet buffer full, discarding oldest packet");
                }
                packets.push_overwrite(response);
            }
            Err(err) => info!("Error when capturing a packet: {:?}", err),
        }
    }
}

async fn send_batch(packets: Vec<Response>, ws_tx: &UnboundedSender<Message>, client_id: usize) {
    let encoded = bincode::serialize(&packets).unwrap_or_else(|e| {
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

        client.queue.push_back(msg);
    }
}

// async fn reparse_packet(
//     client_id: usize,
//     packets: &Packets,
//     clients: &Clients,
//     id: usize,
//     cur_source: &Source,
//     packet_type: SessionProtocol,
// ) {
// let mut packets = packets.write().await;
// let Some(response_packet) = packets.iter_mut().nth(id) else {
//     warn!(
//         "Received reparse request for non-existent packet {}, client_id: {}",
//         id, client_id
//     );
//     return;
// };

// let Response::Packet(packet) = response_packet else {
//     unreachable!("");
// };
// packet.parse_as(packet_type);

// let Ok(encoded) = response_packet.encode() else {
//     error!("Failed to encode packet, client_id: {}", client_id);
//     return;
// };
// let msg = Message::binary(encoded);
// for (_, client) in clients.read().await.iter() {
//     match client {
//         Client {
//             source: Some(source),
//             sender,
//         } if *source == *cur_source => {
//             sender.send(msg.clone()).unwrap_or_else(|e| {
//                 error!("Sniffer: error while sending packet: {}", e);
//             });
//         }
//         _ => {}
//     };
// }
// }

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
                    Request::FetchAll => {
                        if let Some(ref cur_source) = source {
                            if let Some(packets) = packets.get(cur_source) {
                                send_all_packets(client_id, packets, clients).await;
                            } else {
                                warn!(
                                    "No packets found for source: {:?}, client_id: {}",
                                    cur_source, client_id
                                );
                            }
                        }
                    }
                    // Request::Reparse(id, packet_type) => {
                    //     if let Some(ref cur_source) = source {
                    //         if let Some(packets) = packets.get(cur_source) {
                    //             reparse_packet(
                    //                 client_id,
                    //                 packets,
                    //                 clients,
                    //                 id,
                    //                 cur_source,
                    //                 packet_type,
                    //             )
                    //             .await;
                    //         } else {
                    //             warn!(
                    //                 "No packets found for source: {:?}, client_id: {}",
                    //                 cur_source, client_id
                    //             );
                    //         }
                    //     } else {
                    //         error!(
                    //             "Received reparse request without a selected source, client_id: {}",
                    //             client_id
                    //         );
                    //     }
                    // }
                    Request::ChangeSource(new_source) => {
                        if let Some(packets) = packets.get(&new_source) {
                            {
                                let mut wr_clients = clients.write().await;
                                let client = wr_clients.get_mut(&client_id).unwrap();
                                client.source = Some(new_source.clone());
                            }
                            source = Some(new_source);
                            send_all_packets(client_id, packets, clients).await;
                        } else {
                            warn!(
                                "Attempted to change to unknown source: {:?}, client_id: {}",
                                new_source, client_id
                            );
                        }
                    }

                    Request::ParseSdp(stream_key, sdp) => {
                        if let Some(cur_source) = &source {
                            parse_sdp(client_id, clients, cur_source, stream_key, sdp).await;
                        } else {
                            warn!("Received ParseSdp request without a selected source, client_id: {}", client_id);
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
