use crate::server::handler::{handle_messages, send_pcap_filenames};

use super::handler::PacketsMap;
use super::config::Config;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{error, info, warn};
use netpix_common::Source;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use warp::ws::{Message, WebSocket};

static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);
static ACTIVE_CLIENTS: AtomicUsize = AtomicUsize::new(0);

pub struct Client {
    pub sender: UnboundedSender<Message>,
    pub source: Option<Source>,
    pub queue: VecDeque<Message>, // Messages waiting to be sent
    pub max_queue_size: usize,
}

impl Client {
    pub fn new(sender: UnboundedSender<Message>, max_queue_size: usize) -> Self {
        Self {
            sender,
            source: None,
            queue: VecDeque::new(),
            max_queue_size,
        }
    }

    /// Attempt to add a message to the queue with backpressure
    pub fn try_queue_message(&mut self, msg: Message) -> bool {
        if self.queue.len() >= self.max_queue_size {
            warn!("Client queue full, dropping message to prevent memory exhaustion");
            false
        } else {
            self.queue.push_back(msg);
            true
        }
    }
}

pub type Clients = Arc<RwLock<HashMap<usize, Client>>>;

pub fn new_clients() -> Clients {
    Clients::default()
}

pub async fn handle_connection(ws: WebSocket, clients: Clients, packets: PacketsMap, config: Config) {
    // Check if we've reached the maximum number of clients
    let active = ACTIVE_CLIENTS.load(Ordering::Relaxed);
    if active >= config.max_clients {
        warn!("Maximum client limit reached ({}), rejecting new connection", config.max_clients);
        // Close the connection gracefully
        return;
    }

    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    ACTIVE_CLIENTS.fetch_add(1, Ordering::Relaxed);

    info!("New client connected, assigned id: {} (active: {})", client_id, active + 1);

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

    clients.write().await.insert(client_id, Client::new(tx, config.max_client_queue_size));

    handle_messages(client_id, ws_rx, &clients, &packets).await;

    info!("Client disconnected, client_id: {}", client_id);
    clients.write().await.remove(&client_id);
    ACTIVE_CLIENTS.fetch_sub(1, Ordering::Relaxed);
}
