use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub transport_error_indicator: bool,
    pub payload_unit_start_indicator: bool,
    pub transport_priority: bool,
    pub pid: PIDTable,
    pub transport_scrambling_control: TransportScramblingControl,
    pub adaptation_field_control: AdaptationFieldControl,
    pub continuity_counter: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PIDTable {
    ProgramAssociation,
    ConditionalAccess,
    TransportStreamDescription,
    IPMPControlInformation,
    AdaptiveStreamingInformation,
    PID(u16),
    NullPacket,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransportScramblingControl {
    NotScrambled,
    UserDefined(u8),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AdaptationFieldControl {
    PayloadOnly,
    AdaptationFieldOnly,
    AdaptationFieldAndPaylod,
}