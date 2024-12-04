#![allow(dead_code)]
use std::time::Duration;
use std::convert::AsRef;

pub trait StreamStatistics {
    fn get_duration(&self) -> Duration;
    fn get_mean_frame_bitrate(&self) -> f64;
    fn get_mean_protocol_bitrate(&self) -> f64;
    fn get_mean_frame_bytes_rate(&self) -> f64;
    fn get_mean_protocol_bytes_rate(&self) -> f64;
    fn get_mean_packet_rate(&self) -> f64;
    fn update_bitrate(&mut self, bitrate: Bitrate);
    fn update_bytes(&mut self, bytes: Bytes);
    fn update_time(&mut self, time: PacketsTime);
}

#[derive(Debug, Clone, Default)]
pub struct Statistics {
    packets_time: PacketsTime,
    bitrate: Bitrate,
    bytes: Bytes,
    packet_rate: f64,
}
#[derive(Debug, Clone, Default)]
pub struct Bitrate {
    frame_bitrate: f64,
    protocol_bitrate: f64,
}

#[derive(Debug, Clone, Default)]
pub struct Bytes {
    frame_bytes: f64,
    protocol_bytes: f64,
}
#[derive(Debug, Clone, Default)]
pub struct PacketsTime {
    first_time: Duration,
    last_time: Duration,
}

impl AsRef<Statistics> for Statistics {
    fn as_ref(&self) -> &Statistics {
        self
    }
}

impl Statistics {
    pub fn builder() -> StatisticsBuilder {
        StatisticsBuilder::new()
    }

    pub fn get_packets_time(&self) -> &PacketsTime {
        &self.packets_time
    }

    pub fn get_bitrate(&self) -> &Bitrate {
        &self.bitrate
    }

    pub fn get_bytes(&self) -> &Bytes {
        &self.bytes
    }

    pub fn get_packet_rate(&self) -> f64 {
        self.packet_rate
    }

    pub fn set_packet_rate(&mut self, packet_rate: f64) {
        self.packet_rate = packet_rate;
    }

    pub fn increment_packet_rate(&mut self) {
        self.packet_rate += 1.0;
    }

    pub fn set_packets_time(&mut self, packets_time: PacketsTime) {
        self.packets_time = packets_time;
    }

    pub fn set_bitrate(&mut self, bitrate: Bitrate) {
        self.bitrate = bitrate;
    }

    pub fn set_bytes(&mut self, bytes: Bytes) {
        self.bytes = bytes;
    }

    pub fn add_bitrate(&mut self, bitrate: Bitrate) {
        self.bitrate.frame_bitrate += bitrate.frame_bitrate;
        self.bitrate.protocol_bitrate += bitrate.protocol_bitrate;
    }

    pub fn add_bytes(&mut self, bytes: Bytes) {
        self.bytes.frame_bytes += bytes.frame_bytes;
        self.bytes.protocol_bytes += bytes.protocol_bytes;
    }

    pub fn add_time(&mut self, time: PacketsTime) {
        self.packets_time.first_time += time.first_time;
        self.packets_time.last_time += time.last_time;
    }

    pub fn sub_bitrate(&mut self, bitrate: Bitrate) {
        self.bitrate.frame_bitrate -= bitrate.frame_bitrate;
        self.bitrate.protocol_bitrate -= bitrate.protocol_bitrate;
    }

    pub fn sub_bytes(&mut self, bytes: Bytes) {
        self.bytes.frame_bytes -= bytes.frame_bytes;
        self.bytes.protocol_bytes -= bytes.protocol_bytes;
    }

    pub fn sub_time(&mut self, time: PacketsTime) {
        self.packets_time.first_time -= time.first_time;
        self.packets_time.last_time -= time.last_time;
    }

    pub fn clear(&mut self) {
        self.packets_time.clear();
        self.bitrate.clear();
        self.bytes.clear();
    }
}

impl AsRef<Bitrate> for Bitrate {
    fn as_ref(&self) -> &Bitrate {
        self
    }
}
impl Bitrate {
    pub fn builder() -> BitrateBuilder {
        BitrateBuilder::new()
    }

    pub fn get_frame_bitrate(&self) -> f64 {
        self.frame_bitrate
    }

    pub fn get_protocol_bitrate(&self) -> f64 {
        self.protocol_bitrate
    }

    pub fn set_frame_bitrate(&mut self, frame_bitrate: f64) {
        self.frame_bitrate = frame_bitrate;
    }

    pub fn set_protocol_bitrate(&mut self, protocol_bitrate: f64) {
        self.protocol_bitrate = protocol_bitrate;
    }

    pub fn clear(&mut self) {
        self.frame_bitrate = 0.0;
        self.protocol_bitrate = 0.0;
    }
}

impl AsRef<Bytes> for Bytes {
    fn as_ref(&self) -> &Bytes {
        self
    }
}

impl Bytes {
    pub fn builder() -> BytesBuilder {
        BytesBuilder::new()
    }

    pub fn get_frame_bytes(&self) -> f64 {
        self.frame_bytes
    }

    pub fn get_protocol_bytes(&self) -> f64 {
        self.protocol_bytes
    }

    pub fn set_frame_bytes(&mut self, frame_bytes: f64) {
        self.frame_bytes = frame_bytes;
    }

    pub fn set_protocol_bytes(&mut self, protocol_bytes: f64) {
        self.protocol_bytes = protocol_bytes;
    }

    pub fn clear(&mut self) {
        self.frame_bytes = 0.0;
        self.protocol_bytes = 0.0;
    }
}

impl AsRef<PacketsTime> for PacketsTime {
    fn as_ref(&self) -> &PacketsTime {
        self
    }
}
impl PacketsTime {
    pub fn builder() -> PacketsTimeBuilder {
        PacketsTimeBuilder::new()
    }

    pub fn get_first_time(&self) -> Duration {
        self.first_time
    }

    pub fn get_last_time(&self) -> Duration {
        self.last_time
    }

    pub fn set_first_time(&mut self, first_time: Duration) {
        self.first_time = first_time;
    }

    pub fn set_last_time(&mut self, last_time: Duration) {
        self.last_time = last_time;
    }

    pub fn clear(&mut self) {
        self.first_time = Duration::from_secs(0);
        self.last_time = Duration::from_secs(0);
    }
}
pub struct StatisticsBuilder {
    packets_time: PacketsTime,
    bitrate: Bitrate,
    bytes: Bytes,
    packet_rate: f64,
}
pub struct BitrateBuilder {
    frame_bitrate: f64,
    protocol_bitrate: f64,
}
pub struct BytesBuilder {
    frame_bytes: f64,
    protocol_bytes: f64,
}
pub struct PacketsTimeBuilder {
    first_time: Duration,
    last_time: Duration,
}
impl StatisticsBuilder {
    pub fn new() -> Self {
        Self {
            packets_time: PacketsTimeBuilder::new().build(),
            bitrate: BitrateBuilder::new().build(),
            bytes: BytesBuilder::new().build(),
            packet_rate: 0.0,
        }
    }

    pub fn packets_time(mut self, packets_time: PacketsTime) -> Self {
        self.packets_time = packets_time;
        self
    }

    pub fn bitrate(mut self, bitrate: Bitrate) -> Self {
        self.bitrate = bitrate;
        self
    }

    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = bytes;
        self
    }

    pub fn packet_rate(mut self, packet_rate: f64) -> Self {
        self.packet_rate = packet_rate;
        self
    }

    pub fn build(self) -> Statistics {
        Statistics {
            packets_time: self.packets_time,
            bitrate: self.bitrate,
            bytes: self.bytes,
            packet_rate: self.packet_rate,
        }
    }
}
impl BitrateBuilder {
    pub fn new() -> Self {
        Self {
            frame_bitrate: 0.0,
            protocol_bitrate: 0.0,
        }
    }

    pub fn frame_bitrate(mut self, frame_bitrate: f64) -> Self {
        self.frame_bitrate = frame_bitrate;
        self
    }

    pub fn protocol_bitrate(mut self, protocol_bitrate: f64) -> Self {
        self.protocol_bitrate = protocol_bitrate;
        self
    }

    pub fn build(self) -> Bitrate {
        Bitrate {
            frame_bitrate: self.frame_bitrate,
            protocol_bitrate: self.protocol_bitrate,
        }
    }
}
impl BytesBuilder {
    pub fn new() -> Self {
        Self {
            frame_bytes: 0.0,
            protocol_bytes: 0.0,
        }
    }

    pub fn frame_bytes(mut self, frame_bytes: f64) -> Self {
        self.frame_bytes = frame_bytes;
        self
    }

    pub fn protocol_bytes(mut self, protocol_bytes: f64) -> Self {
        self.protocol_bytes = protocol_bytes;
        self
    }

    pub fn build(self) -> Bytes {
        Bytes {
            frame_bytes: self.frame_bytes,
            protocol_bytes: self.protocol_bytes,
        }
    }
}
impl PacketsTimeBuilder {
    pub fn new() -> Self {
        Self {
            first_time: Duration::from_secs(0),
            last_time: Duration::from_secs(0),
        }
    }

    pub fn first_time(mut self, first_time: Duration) -> Self {
        self.first_time = first_time;
        self
    }

    pub fn last_time(mut self, last_time: Duration) -> Self {
        self.last_time = last_time;
        self
    }

    pub fn build(self) -> PacketsTime {
        PacketsTime {
            first_time: self.first_time,
            last_time: self.last_time,
        }
    }
}
