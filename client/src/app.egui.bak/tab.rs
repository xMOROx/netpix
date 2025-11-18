use std::fmt;

pub trait Section: Sized + Copy {
    fn iter() -> impl Iterator<Item = Self>;
    fn display_name(&self) -> String;
}

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
    pub fn all() -> Vec<Self> {
        let mut tabs = vec![Self::Packets];
        tabs.extend(RtpSection::iter().map(Self::RtpSection));
        tabs.extend(MpegTsSection::iter().map(Self::MpegTsSection));
        tabs.extend(IceSection::iter().map(Self::IceSection));
        tabs
    }

    pub fn sections() -> Vec<(String, Vec<Self>)> {
        vec![
            ("📋 General".to_string(), vec![Self::Packets]),
            (
                "🔈 RTP".to_string(),
                RtpSection::iter().map(Self::RtpSection).collect(),
            ),
            (
                "📺 MPEG-TS".to_string(),
                MpegTsSection::iter().map(Self::MpegTsSection).collect(),
            ),
            (
                "🗼 ICE".to_string(),
                IceSection::iter().map(Self::IceSection).collect(),
            ),
        ]
    }

    pub fn from_string(tab_str: String) -> Option<Self> {
        Self::all()
            .into_iter()
            .find(|tab| tab_str == tab.display_name())
    }

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

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packets => write!(f, "📦 Packets"),
            Self::RtpSection(section) => section.fmt(f),
            Self::MpegTsSection(section) => section.fmt(f),
            Self::IceSection(section) => section.fmt(f),
        }
    }
}

impl fmt::Display for RtpSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            Self::Packets => "🔈RTP Packets",
            Self::RtcpPackets => "📃 RTCP Packets",
            Self::Streams => "🔴 RTP Streams",
            Self::Plot => "📈 RTP Plot",
        };

        write!(f, "{}", ret)
    }
}

impl fmt::Display for MpegTsSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            Self::Packets => "📺 MPEG-TS Packets",
            Self::Streams => "🎥 MPEG-TS Streams",
            Self::Information => "ℹ️ MPEG-TS Info",
        };

        write!(f, "{}", ret)
    }
}

impl Section for RtpSection {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::Packets, Self::RtcpPackets, Self::Streams, Self::Plot].into_iter()
    }

    fn display_name(&self) -> String {
        self.to_string()
    }
}

impl Section for MpegTsSection {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::Packets, Self::Streams, Self::Information].into_iter()
    }

    fn display_name(&self) -> String {
        self.to_string()
    }
}

impl Section for IceSection {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::StunPackets].into_iter()
    }

    fn display_name(&self) -> String {
        match self {
            Self::StunPackets => "🔄 🌐 STUN Packets".to_string(),
        }
    }
}

impl fmt::Display for IceSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
