use std::net::SocketAddr;
use std::time::Duration;
use egui_plot::PlotPoint;
use netpix_common::rtcp::{ReceiverReport, ReceptionReport};
use netpix_common::{Packet, RtcpPacket};
use crate::streams::rtpStream::RtcpInfo;
use crate::utils::ntp_to_f64;

#[derive(Debug, Clone)]
pub struct RtcpStream {
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub ssrc: u32,
    // --- Data from Sender Reports (for Bitrate) ---
    /// Timestamp of the last received SR packet for bitrate calculation
    pub last_sr_timestamp: Option<Duration>,
    /// Octet count from the last received SR packet
    pub last_sr_octet_count: Option<u32>,
    /// Calculated average bitrate in bits per second
    pub current_avg_bitrate_bps: f64,
    /// Historical bitrate data points for plotting
    pub bitrate_history: Vec<PlotPoint>,

    // --- Data from Receiver Reports (for Loss) ---
    /// Latest cumulative packets lost value received in an RR
    pub cumulative_lost: Option<u32>,
    /// Latest fraction lost value (scaled 0.0 to 1.0)
    pub fraction_lost: Option<f32>,
    /// Historical cumulative loss data points for plotting
    pub loss_history: Vec<PlotPoint>,
}

impl RtcpStream {
    pub fn new(ssrc: u32, packet: &Packet) -> Self {
        Self {
            source_addr: packet.source_addr,
            destination_addr: packet.destination_addr,
            ssrc,
            last_sr_timestamp: None,
            last_sr_octet_count: None,
            current_avg_bitrate_bps: 0.0,
            bitrate_history: Vec::new(),
            cumulative_lost: None,
            fraction_lost: None,
            loss_history: Vec::new(),
        }
    }

    pub fn add_rtcp_packet(&mut self, _id: usize, timestamp: Duration, packet: &RtcpPacket) {
        match packet {
            RtcpPacket::SenderReport(sr) => {
                if sr.ssrc == self.ssrc {
                    self.update_with_sr(timestamp, sr.octet_count, sr.packet_count);
                }
            }
            RtcpPacket::ReceiverReport(rr) => {
                self.update_with_rr(timestamp,rr)
            }
            _ => {}
        }
    }


    // Methods to update data based on received SR/RR packets would go here
    // update_with_sr(&mut self, timestamp: Duration, octet_count: u32, packet_count: u32)
    // update_with_rr(&mut self, timestamp: Duration, report: &ReceptionReport)
    pub fn update_with_sr(&mut self, timestamp: Duration, octet_count: u32, packet_count: u32) {
        // packet_count is part of SR but not directly used for this bitrate calculation
        let _ = packet_count;

        if let (Some(last_ts), Some(last_octets)) = (self.last_sr_timestamp, self.last_sr_octet_count) {
            let delta_time = timestamp.saturating_sub(last_ts); // Avoid panic on unusual timestamps
            let delta_time_secs = delta_time.as_secs_f64();

            // Only calculate bitrate if time has actually passed to avoid division by zero
            // and if octet count has changed (using wrapping_sub for correctness).
            if delta_time_secs > 0.0 {
                // Use wrapping subtraction to handle potential u32 wrap-around of octet count
                let delta_octets = octet_count.wrapping_sub(last_octets);

                // Calculate bitrate = (delta_bytes * 8) / delta_seconds
                let bitrate = (delta_octets as f64 * 8.0) / delta_time_secs;
                self.current_avg_bitrate_bps = bitrate;

                // Add data point for plotting (time in seconds vs bitrate in bps)
                self.bitrate_history.push(PlotPoint::new(timestamp.as_secs_f64(), bitrate));
            }
        }

        // Always store the latest SR info for the *next* calculation
        self.last_sr_timestamp = Some(timestamp);
        self.last_sr_octet_count = Some(octet_count);
    }

    /// Updates the stream's state based on a Reception Report (RR) block.
    /// Call this when an RR block *reporting about* this stream's SSRC is received
    /// (found within either an RR or SR packet sent by *another* SSRC).
    pub fn update_with_rr(&mut self, timestamp: Duration, rr: &ReceiverReport) {
        // Ensure the report is actually for this SSRC before updating
        if rr.ssrc != self.ssrc {
            // This check might be redundant if the caller already ensures this,
            // but it adds safety.
            return;
        }

        for report in &rr.reports {
            // Cumulative lost is the total count, so we just store the latest value.
            let new_cumulative_lost = report.total_lost;
            self.cumulative_lost = Some(new_cumulative_lost);

            // Fraction lost is an 8-bit fixed point number (0-255 representing 0/256 to 255/256).
            // Convert it to a float between 0.0 and 1.0.
            let new_fraction_lost = report.fraction_lost as f32 / 256.0;
            self.fraction_lost = Some(new_fraction_lost);

            // Add data point for plotting (time in seconds vs cumulative packets lost)
            self.loss_history.push(PlotPoint::new(timestamp.as_secs_f64(), new_cumulative_lost as f64));
        }
    }

}
