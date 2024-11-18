use std::net::SocketAddr;
use crate::packet::TransportProtocol;

pub type MpegtsStreamKey = (SocketAddr, SocketAddr, TransportProtocol);
pub type RtpStreamKey = (SocketAddr, SocketAddr, TransportProtocol, u32);
