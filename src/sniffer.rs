use futures_util::StreamExt;
use log_parser::parser::Parser;
use netpix_common::{Packet, Source};
use pcap::{Capture, PacketCodec, PacketStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

const LOG_CHANNEL_BUFFER_SIZE: usize = 100;

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
    receiver: Receiver<Result<Packet, tokio::time::error::Error>>,
}

impl LogStream {
    pub fn new(receiver: Receiver<Result<Packet, tokio::time::error::Error>>) -> Self {
        Self { receiver }
    }

    pub async fn next(&mut self) -> Option<Result<Packet, tokio::time::error::Error>> {
        self.receiver.recv().await
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
        let (tx, rx) = mpsc::channel(LOG_CHANNEL_BUFFER_SIZE);

        let file_path = file.to_string();
        tokio::spawn(async move {
            if let Err(e) = Parser::watch_log_file(file_path, tx).await {
                eprintln!("Error watching log file: {:?}", e);
            }
        });

        let log_stream = LogStream::new(rx);

        Ok(Self {
            capture: CaptureType::RtcLogging(log_stream),
            source: Source::File(file.to_string()),
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
        let packet_result = match self.capture {
            CaptureType::Offline(ref mut stream) => stream.next().map(|res| {
                res.map_err(|_arg0: pcap::Error| Error::from(Error::CouldntReceivePacket))
                    .and_then(|inner_res| inner_res)
            }),

            CaptureType::Online(ref mut stream) => stream.next().await.map(|res| {
                res.map_err(|_arg0: pcap::Error| Error::from(Error::CouldntReceivePacket))
                    .and_then(|inner_res| inner_res)
            }),

            CaptureType::RtcLogging(ref mut stream) => stream.next().await.map(|res| {
                res.map_err(|_arg0: tokio::time::error::Error| {
                    Error::from(Error::CouldntReceivePacket)
                })
            }),
        };

        packet_result
    }
}
