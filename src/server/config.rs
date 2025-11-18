use bon::Builder;
use std::net::SocketAddr;

#[derive(Debug, Builder, Clone, Copy)]
pub struct Config {
    pub max_packets_age: u64,
    pub client_message_interval_ms: u64,
    pub packet_buffer_size: usize,
    pub addr: SocketAddr,
    /// Maximum number of messages that can be queued per client before backpressure is applied
    #[builder(default = 1000)]
    pub max_client_queue_size: usize,
    /// Maximum number of concurrent client connections
    #[builder(default = 100)]
    pub max_clients: usize,
}
