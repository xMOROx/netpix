use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Packets,
    RtpSection(RtpSection),
    MpegTsSection(MpegTsSection),
    IceSection(IceSection),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RtpSection {
    Packets,
    RtcpPackets,
    Streams,
    Plot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MpegTsSection {
    Packets,
    Streams,
    Information,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IceSection {
    StunPackets,
}

impl Tab {
    pub fn display_name(&self) -> String {
        match self {
            Self::Packets => "📦 Packets".to_string(),
            Self::RtpSection(section) => section.display_name(),
            Self::MpegTsSection(section) => section.display_name(),
            Self::IceSection(section) => section.display_name(),
        }
    }

    pub fn get_table_id(&self) -> &'static str {
        match self {
            Tab::Packets => "packets",
            Tab::RtpSection(section) => match section {
                RtpSection::Packets => "rtp_packets",
                RtpSection::RtcpPackets => "rtcp_packets",
                RtpSection::Streams => "rtp_streams",
                RtpSection::Plot => "rtp_streams_plot",
            },
            Tab::MpegTsSection(section) => match section {
                MpegTsSection::Packets => "mpegts_packets",
                MpegTsSection::Streams => "mpegts_streams",
                MpegTsSection::Information => "mpegts_info",
            },
            Tab::IceSection(section) => match section {
                IceSection::StunPackets => "stun_packets",
            },
        }
    }
}

impl RtpSection {
    pub fn display_name(&self) -> String {
        match self {
            RtpSection::Packets => "🔈 RTP Packets".to_string(),
            RtpSection::RtcpPackets => "🔈 RTCP Packets".to_string(),
            RtpSection::Streams => "🔈 RTP Streams".to_string(),
            RtpSection::Plot => "🔈 RTP Plot".to_string(),
        }
    }
}

impl MpegTsSection {
    pub fn display_name(&self) -> String {
        match self {
            MpegTsSection::Packets => "📺 MPEG-TS Packets".to_string(),
            MpegTsSection::Streams => "📺 MPEG-TS Streams".to_string(),
            MpegTsSection::Information => "📺 MPEG-TS Information".to_string(),
        }
    }
}

impl IceSection {
    pub fn display_name(&self) -> String {
        match self {
            IceSection::StunPackets => "🗼 STUN Packets".to_string(),
        }
    }
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
