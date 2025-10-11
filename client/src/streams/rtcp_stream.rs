use crate::streams::rtpStream::RtcpInfo;
use crate::utils::{ntp_to_f64, ntp_to_unix_time};
use chrono::{Duration, TimeDelta};
use core::time;
use egui_plot::PlotPoint;
use netpix_common::rtcp::{ReceiverReport, ReceptionReport, SenderReport};
use netpix_common::{Packet, RtcpPacket};
use std::net::SocketAddr;
use std::ops::Div;

#[derive(Debug, Clone)]
pub struct RtcpStream {
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub ssrc: u32,
    // --- Data from Sender Reports
    pub last_sr_timestamp: Option<Duration>,
    pub last_sr_octet_count: Option<u32>,
    pub current_avg_bitrate_bps: f64,
    pub bitrate_history: Vec<(u64, f64)>,

    // --- Data from Receiver Reports---
    pub cumulative_lost: Option<u32>,
    pub fraction_lost: Option<f32>,
    pub loss_history: Vec<PlotPoint>,
    pub jitter_history: Vec<PlotPoint>,
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
            jitter_history: Vec::new(),
        }
    }

    pub fn update_with_sr(&mut self, packet: &RtcpPacket) {
        if let RtcpPacket::SenderReport(report) = packet {
            if report.ssrc == self.ssrc {
                let current_event_time_duration = ntp_to_unix_time(report.ntp_time);

                if let (Some(last_event_time_duration), Some(last_octets)) =
                    (self.last_sr_timestamp, self.last_sr_octet_count)
                {
                    if let Some(delta_duration) =
                        current_event_time_duration.checked_sub(&last_event_time_duration)
                    {
                        let delta_time_secs =
                            delta_duration.num_microseconds().unwrap_or(0) as f64 / 1_000_000.0;

                        if delta_time_secs > 0.0 {
                            let delta_octets = report.octet_count.wrapping_sub(last_octets);

                            let bitrate_bps = (delta_octets as f64 * 8.0) / delta_time_secs;
                            self.current_avg_bitrate_bps = bitrate_bps;

                            self.bitrate_history.push((report.ntp_time, bitrate_bps));
                        }
                    }
                }
                self.last_sr_timestamp = Some(current_event_time_duration);
                self.last_sr_octet_count = Some(report.octet_count);
            }
        }
    }

    pub fn update_with_rr(&mut self, timestamp: time::Duration, report: &ReceptionReport) {
        // Cumulative lost is the total count, so we just store the latest value.
        let new_cumulative_lost = report.total_lost;
        self.cumulative_lost = Some(new_cumulative_lost);

        // Fraction lost is an 8-bit fixed point number (0-255 representing 0/256 to 255/256).
        // Convert it to a float between 0.0 and 1.0.
        let new_fraction_lost = report.fraction_lost as f32 / 256.0;
        self.fraction_lost = Some(new_fraction_lost);

        // Add data point for plotting (time in seconds vs cumulative packets lost)
        self.loss_history.push(PlotPoint::new(
            timestamp.as_secs_f64(),
            new_cumulative_lost as f64,
        ));

        self.jitter_history.push(PlotPoint::new(
            timestamp.as_secs_f64(),
            report.jitter as f64,
        ));
    }
}
