use std::collections::HashMap;
use crate::bitstream::{BlobDecoder, FixedLengthDeltaDecoder};
use crate::types::{LogRtcpPacket, RtcpPacketType};
use crate::webrtc::rtclog2::{AudioRecvStreamConfig, AudioSendStreamConfig, EventStream, VideoRecvStreamConfig, VideoSendStreamConfig};
use netpix_common::packet::{Packet,PacketDirection, PacketMetadata, SessionPacket, SessionProtocol, TransportProtocol, StreamType,StreamMetaData};
use prost::{DecodeError, Message};
use std::io::SeekFrom;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::SystemTime;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::sync::mpsc;
use tokio::time::error::Error;
use tokio::time::{Duration, sleep};

const READ_BUFFER_SIZE: usize = 1024;
const POLL_INTERVAL_MS: u64 = 200;

pub struct Parser {
    pub packets: Vec<Packet>,
    pub stream_meta: HashMap<u32,StreamMetaData>,
    pack_num: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            packets: vec![],
            stream_meta: HashMap::new(),
            pack_num: 0,
        }
    }

    fn decode(&mut self, buf: &[u8]) -> Result<(), DecodeError> {
        let event_stream: EventStream = Message::decode(buf)?;

        self.parse_video_send_stream_config(&event_stream.video_send_stream_configs);
        self.parse_video_recv_stream_config(&event_stream.video_recv_stream_configs);
        self.parse_audio_send_stream_config(&event_stream.audio_send_stream_configs);
        self.parse_audio_recv_stream_config(&event_stream.audio_recv_stream_configs);

        let inc_packets: Vec<LogRtcpPacket> = event_stream
            .incoming_rtcp_packets
            .into_iter()
            .map(Into::into)
            .collect();

        match self.parse_rtcp_packets(inc_packets) {
            Ok(()) => {}
            Err(e) => return Err(e),
        }

        let out_packets: Vec<LogRtcpPacket> = event_stream
            .outgoing_rtcp_packets
            .into_iter()
            .map(Into::into)
            .collect();

        match self.parse_rtcp_packets(out_packets) {
            Ok(()) => {}
            Err(e) => return Err(e),
        };

        self.packets.sort_by_key(|p| p.timestamp);

        for (i, packet) in self.packets.iter_mut().enumerate() {
            packet.id = i + self.pack_num;
        }

        Ok(())
    }

    pub async fn watch_log_file(
        file_path: String,
        tx: mpsc::Sender<Result<Packet, Error>>,
    ) -> Result<(), std::io::Error> {
        let mut file = File::open(file_path).await?;
        file.seek(SeekFrom::Start(0)).await?;

        let mut parser = Parser::new();

        let mut buf = vec![0u8; READ_BUFFER_SIZE];

        loop {
            let mut persistent_buf = Vec::new();
            let mut bytes_read = file.read(&mut buf[..]).await?;

            if bytes_read > 0 {
                while bytes_read > 0 {
                    persistent_buf.extend_from_slice(&buf[..bytes_read]);
                    bytes_read = file.read(&mut buf[..]).await?;
                }

                if parser.decode(&persistent_buf).is_ok() {
                    parser.pack_num += parser.packets.len();
                    for packet in parser.packets.drain(0..) {
                        if tx.send(Ok(packet)).await.is_err() {
                            println!("Receiver dropped. Stopping log watcher.");
                            return Ok(());
                        }
                    }
                }
            } else {
                // inefficient workaround because: https://github.com/notify-rs/notify/issues/254
                sleep(Duration::from_millis(POLL_INTERVAL_MS)).await;
            }
        }
    }

    pub fn parse_rtcp_packets(
        &mut self,
        rtcp_packets: Vec<LogRtcpPacket>,
    ) -> Result<(), DecodeError> {
        let inc_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let out_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8080);

        for packets in rtcp_packets {
            'packet_group: {
                let Some(timestamp_deltas) = &packets.timestamp_ms_deltas else {
                    eprintln!("Warning: Skipping packet group due to missing timestamp deltas.");
                    break 'packet_group;
                };

                let Some(base_timestamp) = packets.timestamp_ms else {
                    eprintln!("Warning: Skipping packet group due to missing base timestamp.");
                    break 'packet_group;
                };

                let Some(num_deltas) = packets.number_of_deltas else {
                    eprintln!("Warning: Skipping packet group due to missing number of deltas.");
                    break 'packet_group;
                };

                let Some(raw_blobs) = &packets.raw_packet_blobs else {
                    eprintln!("Warning: Skipping packet group due to missing raw packet blobs.");
                    break 'packet_group;
                };

                let mut timestamp_decoder = match FixedLengthDeltaDecoder::new(
                    timestamp_deltas,
                    base_timestamp as u64,
                    num_deltas as usize,
                ) {
                    Ok(decoder) => decoder,
                    Err(_) => {
                        eprintln!(
                            "Warning: Could not create timestamp decoder. Skipping packet group."
                        );
                        break 'packet_group;
                    }
                };

                let timestamps = match timestamp_decoder.decode() {
                    Ok(values) => values,
                    Err(_) => {
                        eprintln!(
                            "Warning: Timestamps could not be decoded. Skipping packet group."
                        );
                        break 'packet_group;
                    }
                };

                let mut blob_decoder = BlobDecoder::new(raw_blobs, num_deltas as usize);
                let blobs = match blob_decoder.decode() {
                    Ok(decoded_blobs) => decoded_blobs,
                    Err(_) => {
                        eprintln!(
                            "Warning: RTCP blobs could not be decoded. Skipping packet group."
                        );
                        break 'packet_group;
                    }
                };

                let timestamp = Duration::from_millis(packets.timestamp_ms.unwrap() as u64);
                let payload = packets.raw_packet.unwrap();
                let length = payload.len();



                let (source_addr, destination_addr, metadata) = match packets.type_ {
                    RtcpPacketType::Outgoing => (
                        out_addr,
                        inc_addr,
                        PacketMetadata {
                            direction: PacketDirection::Outgoing,
                            is_synthetic_addr: true,
                            stream_meta_data: None
                        }
                    ),
                    _ => (
                        inc_addr,
                        out_addr,
                        PacketMetadata {
                            direction: PacketDirection::Incoming,
                            is_synthetic_addr: true,
                            stream_meta_data: None
                        }
                    ),
                };

                self.packets.push(Packet {
                    payload: Some(payload),
                    id: 0,
                    timestamp,
                    length: length as u32,
                    source_addr,
                    destination_addr,
                    transport_protocol: TransportProtocol::Udp,
                    session_protocol: SessionProtocol::Rtcp,
                    contents: SessionPacket::Unknown,
                    creation_time: SystemTime::now(),
                    metadata: metadata.clone(),
                });

                for (i, blob) in blobs.iter().enumerate() {
                    let Some(timestamp_ms) = timestamps.get(i).and_then(|&t| t) else {
                        eprintln!("Warning: Missing timestamp for a packet, skipping it.");
                        continue;
                    };

                    let payload = blob.to_vec();
                    let length = payload.len() as u32;

                    self.packets.push(Packet {
                        payload: Some(payload),
                        id: 0,
                        timestamp: Duration::from_millis(timestamp_ms),
                        length,
                        source_addr,
                        destination_addr,
                        transport_protocol: TransportProtocol::Udp,
                        session_protocol: SessionProtocol::Rtcp,
                        contents: SessionPacket::Unknown,
                        creation_time: SystemTime::now(),
                        metadata: metadata.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    fn register_stream(&mut self, ssrc_opt: Option<u32>, stream_type: StreamType) {
        if let Some(ssrc) = ssrc_opt {
            self.stream_meta.insert(
                ssrc,
                StreamMetaData { ssrc, stream_type }
            );
        }
    }

    pub fn parse_video_send_stream_config(&mut self, configs: &[VideoSendStreamConfig]) {
        for config in configs {
            self.register_stream(config.ssrc, StreamType::Video);
            self.register_stream(config.rtx_ssrc, StreamType::RTX);
        }
    }

    pub fn parse_video_recv_stream_config(&mut self, configs: &[VideoRecvStreamConfig]) {
        for config in configs {
            self.register_stream(config.remote_ssrc, StreamType::Video);
            self.register_stream(config.local_ssrc, StreamType::VideoControl);
            self.register_stream(config.rtx_ssrc, StreamType::RTX);
        }
    }

    pub fn parse_audio_send_stream_config(&mut self, configs: &[AudioSendStreamConfig]) {
        for config in configs {
            self.register_stream(config.ssrc, StreamType::Audio);
        }
    }

    pub fn parse_audio_recv_stream_config(&mut self, configs: &[AudioRecvStreamConfig]) {
        for config in configs {
            self.register_stream(config.remote_ssrc, StreamType::Audio);
            self.register_stream(config.local_ssrc, StreamType::AudioControl);
        }
    }
}


