use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandidatePairKey {
    pub local_candidate: String,
    pub remote_candidate: String,
}

#[derive(Debug, Clone)]
pub struct CandidatePairStats {
    pub key: CandidatePairKey,
    pub state: CandidatePairState,
    pub checks_sent: u32,
    pub checks_received: u32,
    pub responses_received: u32,
    pub responses_sent: u32,
    pub failures: u32,
    pub avg_rtt_ms: f64,
    pub first_seen: Duration,
    pub last_seen: Duration,
    pub media_packets_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidatePairState {
    Gathering,
    Gathered,
    InProgress,
    Nominated,
    SendingMedia,
    Failed,
    Frozen,
    Disconnected,
}

impl CandidatePairState {
    pub fn color(&self) -> egui::Color32 {
        match self {
            CandidatePairState::Gathering => egui::Color32::GRAY,
            CandidatePairState::Nominated => egui::Color32::GREEN,
            CandidatePairState::SendingMedia => egui::Color32::from_rgb(0, 200, 255),
            CandidatePairState::InProgress => egui::Color32::YELLOW,
            CandidatePairState::Failed => egui::Color32::RED,
            CandidatePairState::Gathered => egui::Color32::LIGHT_BLUE,
            CandidatePairState::Frozen => egui::Color32::DARK_GRAY,
            CandidatePairState::Disconnected => egui::Color32::LIGHT_GRAY,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            CandidatePairState::Gathering => "> Gathering",
            CandidatePairState::Nominated => "+ Nominated",
            CandidatePairState::SendingMedia => "▶ Sending Media",
            CandidatePairState::InProgress => "~ Checking",
            CandidatePairState::Failed => "X Failed",
            CandidatePairState::Gathered => "* Gathered",
            CandidatePairState::Frozen => "# Frozen",
            CandidatePairState::Disconnected => "- Disconnected",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            CandidatePairState::Gathering => "Discovering public IP/port via STUN server",
            CandidatePairState::Gathered => "Gathered server reflexive address",
            CandidatePairState::InProgress => "Performing peer-to-peer connectivity checks",
            CandidatePairState::Nominated => "Pair nominated for communication",
            CandidatePairState::SendingMedia => "Actively sending RTP/RTCP media",
            CandidatePairState::Failed => "Connectivity check failed",
            CandidatePairState::Frozen => "No information yet",
            CandidatePairState::Disconnected => "Connection lost",
        }
    }
}

#[derive(Default)]
pub struct IceCandidatesData {
    pub candidate_pairs: HashMap<CandidatePairKey, CandidatePairStats>,
}
