use crate::bitstream::{BlobDecoder, FixedLengthDeltaDecoder};
use crate::types::{LogRtcpPacket, RtcpPacketType};
use crate::webrtc::rtclog2::EventStream;
use netpix_common::packet::{Packet, SessionPacket, SessionProtocol, TransportProtocol};
use prost::{DecodeError, Message};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::{Duration, SystemTime};

pub struct Parser {
    pub packets: Vec<Packet>,
}

impl Parser {
    pub fn new(packets: Vec<Packet>) -> Parser {
        Parser { packets }
    }

    pub fn decode_from_file(&mut self, file_path: String) -> Result<(), DecodeError> {
        let Ok(mut file) = File::open(file_path.clone()) else {
            return Err(DecodeError::new("File not found"));
        };
        let Ok(_) = file.seek(SeekFrom::Start(0)) else {
            return Err(DecodeError::new("Could not seek to the end of the file"));
        };

        let mut buf = [0u8; 1024 * 1024];

        println!("Watching for new content in '{}'...", file_path);

        loop {
            // file.read() attempts to fill the buffer with bytes from the file.
            // It returns the number of bytes actually read.
            let bytes_read = file.read(&mut buf)
                .map_err(|_| DecodeError::new("Error reading from file"))?;

            // 5. If we read new bytes, process them.
            if bytes_read > 0 {
                // The new data is the slice of the buffer from the start
                // up to the number of bytes read.
                let new_data_slice = &buf[..bytes_read];

                // Pass ONLY the new data to the decode function.
                self.decode(new_data_slice)?;
                println!("packets now: {:?}", self.packets.len());
                // =========================================================

            } else {
                // 6. If there's no new content (read returned 0 bytes), sleep briefly.
                // This prevents the loop from consuming 100% CPU.
                thread::sleep(Duration::from_millis(200));
            }
        }
    }

    fn decode(&mut self, mut buf: &[u8]) -> Result<(), DecodeError> {
        let event_stream: EventStream = Message::decode(buf)?;
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
            packet.id = i;
        }

        Ok(())
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

                let (source_addr, destination_addr) = match packets.type_ {
                    RtcpPacketType::Outgoing => (out_addr, inc_addr),
                    _ => (inc_addr, out_addr),
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
                        id: 0, // Assign the unique, incrementing ID.
                        timestamp: Duration::from_millis(timestamp_ms),
                        length,
                        source_addr,
                        destination_addr,
                        transport_protocol: TransportProtocol::Udp,
                        session_protocol: SessionProtocol::Rtcp,
                        contents: SessionPacket::Unknown,
                        creation_time: SystemTime::now(),
                    });
                }
            }
        }
        Ok(())
    }
}
