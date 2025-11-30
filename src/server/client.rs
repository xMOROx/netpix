use crate::server::handler::{handle_messages, send_pcap_filenames};

use super::handler::PacketsMap;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{error, info};
use netpix_common::Source;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::sync::{
    RwLock,
    mpsc::{self, UnboundedSender},
};
use warp::ws::{Message, WebSocket};

static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

pub struct Client {
    pub sender: UnboundedSender<Message>,
    pub source: Option<Source>,
    pub queue: VecDeque<Message>, // Messages waiting to be sent
}

impl Client {
    pub fn new(sender: UnboundedSender<Message>) -> Self {
        Self {
            sender,
            source: None,
            queue: VecDeque::new(),
        }
    }
}

pub type Clients = Arc<RwLock<HashMap<usize, Client>>>;

pub fn new_clients() -> Clients {
    Clients::default()
}

pub async fn handle_connection(ws: WebSocket, clients: Clients, packets: PacketsMap) {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    info!("New client connected, assigned id: {}", client_id);

    let (mut ws_tx, ws_rx) = ws.split();

    send_pcap_filenames(&client_id, &mut ws_tx, &packets).await;

    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        while let Some(message) = rx.recv().await {
            ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    error!("WebSocket `send` error: {}, client_id: {}", e, client_id);
                })
                .await;
        }
    });

    clients.write().await.insert(client_id, Client::new(tx));

    handle_messages(client_id, ws_rx, &clients, &packets).await;

    info!("Client disconnected, client_id: {}", client_id);
    clients.write().await.remove(&client_id);
}
