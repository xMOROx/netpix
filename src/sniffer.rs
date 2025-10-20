use futures_util::StreamExt;
use log_parser::parser::Parser;
use netpix_common::{Packet, Source};
use pcap::{Capture, PacketCodec, PacketStream};

#[derive(Debug)]
pub enum Error {
    CouldntReceivePacket,
    FileNotFound,
    DeviceNotFound,
    DeviceUnavailable,
    UnsupportedPacketType,
    InvalidFilter,
    PacketStreamUnavailable,
}
#[derive(Debug)]
struct PacketDecoder {
    packet_id: usize,
}

impl PacketDecoder {
    pub fn new() -> Self {
        Self { packet_id: 1 }
    }
}

impl PacketCodec for PacketDecoder {
    type Item = Result<Packet, Error>;

    fn decode(&mut self, packet: pcap::Packet<'_>) -> Self::Item {
        let res = match Packet::build(&packet, self.packet_id) {
            Some(packet) => Ok(packet),
            None => Err(Error::UnsupportedPacketType),
        };

        self.packet_id += 1;
        res
    }
}

// well, it's not technically a Stream...
struct OfflineStream {
    capture: Capture<pcap::Offline>,
    decoder: PacketDecoder,
}

impl OfflineStream {
    pub fn new(capture: Capture<pcap::Offline>, decoder: PacketDecoder) -> Self {
        Self { capture, decoder }
    }

    pub fn next(&mut self) -> Option<Result<Result<Packet, Error>, pcap::Error>> {
        let packet = match self.capture.next_packet() {
            Err(pcap::Error::NoMorePackets) => return None,
            Err(err) => return Some(Err(err)),
            Ok(packet) => packet,
        };

        Some(Ok(self.decoder.decode(packet)))
    }
}

struct LogStream {
    packets: Vec<Packet>,
    cursor: usize,
}

impl LogStream {
    pub fn new(packets: Vec<Packet>) -> Self {
        Self {
            packets,
            cursor: 0usize,
        }
    }
    pub fn next(&mut self) -> Option<Result<Result<Packet, Error>, pcap::Error>> {
        if self.cursor < self.packets.len() {
            let pkt = self.packets[self.cursor].clone();
            self.cursor += 1;

            Some(Ok(Ok(pkt)))
        } else {
            None
        }
    }
}

enum CaptureType {
    Offline(OfflineStream),
    Online(PacketStream<pcap::Active, PacketDecoder>),
    RtcLogging(LogStream),
}

pub struct Sniffer {
    capture: CaptureType,
    pub source: Source,
}

impl Sniffer {
    pub fn from_file(file: &str) -> Result<Self, Error> {
        let Ok(capture) = pcap::Capture::from_file(file) else {
            return Err(Error::FileNotFound);
        };

        let decoder = PacketDecoder::new();
        let stream = OfflineStream::new(capture, decoder);

        Ok(Self {
            capture: CaptureType::Offline(stream),
            source: Source::File(file.to_string()),
        })
    }

    pub fn from_device(device: &str, promisc: bool) -> Result<Self, Error> {
        let Ok(capture) = pcap::Capture::from_device(device) else {
            return Err(Error::DeviceNotFound);
        };

        let Ok(capture) = capture.immediate_mode(true).promisc(promisc).open() else {
            return Err(Error::DeviceUnavailable);
        };

        let Ok(capture) = capture.setnonblock() else {
            return Err(Error::DeviceUnavailable);
        };

        let decoder = PacketDecoder::new();
        let Ok(stream) = capture.stream(decoder) else {
            return Err(Error::PacketStreamUnavailable);
        };

        Ok(Self {
            capture: CaptureType::Online(stream),
            source: Source::Interface(format!("{} {}", device, if promisc { "👁️" } else { "" })),
        })
    }

    pub fn from_logs(file: &str) -> Result<Self, Error> {
        let mut parser = Parser::new(Vec::new());
        parser
            .decode_from_file(file.to_string())
            .expect("Failed to decode from log file");

        let log_stream = LogStream::new(parser.packets);

        Ok(Self {
            capture: CaptureType::RtcLogging(log_stream),
            source: Source::Interface(file.to_string()),
        })
    }

    pub fn apply_filter(&mut self, filter: &str) -> Result<(), Error> {
        match self.capture {
            CaptureType::Online(ref mut stream) => stream.capture_mut().filter(filter, true),
            CaptureType::Offline(ref mut stream) => stream.capture.filter(filter, true),
            CaptureType::RtcLogging(_) => Ok(()),
        }
        .map_err(|_| Error::InvalidFilter)
    }

    pub async fn next_packet(&mut self) -> Option<Result<Packet, Error>> {
        let packet = match self.capture {
            CaptureType::Offline(ref mut stream) => stream.next(),
            CaptureType::Online(ref mut stream) => stream.next().await,
            CaptureType::RtcLogging(ref mut stream) => stream.next(),
        };

        match packet {
            None => None,
            Some(Err(_)) => Some(Err(Error::CouldntReceivePacket)),
            Some(Ok(pack)) => Some(pack),
        }
    }
}
