use bon::Builder;
use std::net::SocketAddr;

#[derive(Debug, Builder, Clone, Copy)]
pub struct Config {
    pub max_packets_age: u64,
    pub client_message_interval_ms: u64,
    pub packet_buffer_size: usize,
    pub addr: SocketAddr,
}
