use std::fmt;
use std::net::SocketAddr;
use crate::packet::TransportProtocol;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PacketAssociationTable {
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub protocol: TransportProtocol,
}

pub type MpegtsStreamKey = (SocketAddr, SocketAddr, TransportProtocol);
pub type RtpStreamKey = (SocketAddr, SocketAddr, TransportProtocol, u32);

impl fmt::Display for PacketAssociationTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} -> {} ({:?})",
            self.source_addr, self.destination_addr, self.protocol
        )
    }
}
