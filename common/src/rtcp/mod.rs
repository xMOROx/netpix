pub mod goodbye;
pub mod receiver_report;
pub mod reception_report;
pub mod sender_report;
pub mod source_description;
pub mod payload_feedbacks;
pub mod transport_feedback;

use bincode::{Decode, Encode};
#[cfg(not(target_arch = "wasm32"))]
pub use ::rtcp::header::PacketType;
pub use goodbye::Goodbye;
pub use receiver_report::ReceiverReport;
pub use reception_report::ReceptionReport;
pub use sender_report::SenderReport;
pub use source_description::SourceDescription;
pub use transport_feedback::TransportFeedback;
pub use payload_feedbacks::{
    fir_entry::FirEntry,
    full_intra_request::FullIntraRequest,
    picture_loss_indication::PictureLossIndication,
    receiver_estimated_maximum_bitrate::ReceiverEstimatedMaximumBitrate,
    slice_loss_indication::SliceLossIndication,
    sli_entry::SliEntry,
};
use crate::rtcp::payload_feedbacks::*;

#[derive(Decode, Encode, Debug, Clone)]
pub enum RtcpPacket {
    SenderReport(SenderReport),
    ReceiverReport(ReceiverReport),
    SourceDescription(SourceDescription),
    Goodbye(Goodbye),
    PictureLossIndication(PictureLossIndication),
    ReceiverEstimatedMaximumBitrate(ReceiverEstimatedMaximumBitrate),
    SliceLossIndication(SliceLossIndication),
    FullIntraRequest(FullIntraRequest),
    ApplicationDefined,
    PayloadSpecificFeedback,
    TransportSpecificFeedback(TransportFeedback),
    ExtendedReport,
    Other(u8),
}

impl RtcpPacket {
    pub fn get_type_name(&self) -> &str {
        use RtcpPacket::*;

        match self {
            SenderReport(_) => "Sender Report",
            ReceiverReport(_) => "Receiver Report",
            SourceDescription(_) => "Source Description",
            Goodbye(_) => "Goodbye",
            PictureLossIndication(_) => "Picture Loss Indication",
            ReceiverEstimatedMaximumBitrate(_) => "Receiver Estimated Maximum Bitrate",
            SliceLossIndication(_) => "Slice Loss Indication",
            FullIntraRequest(_) => "Full Intra Request",
            ApplicationDefined => "Application Defined",
            PayloadSpecificFeedback => "Payload-specific Feedback",
            TransportSpecificFeedback(_) => "Transport-specific Feedback",
            ExtendedReport => "Extended Report",
            Other(_) => "Other",
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl RtcpPacket {
    pub fn build(packet: &super::Packet) -> Option<Vec<Self>> {
        use rtcp::packet;

        // payload field should never be empty
        // except for when encoding the packet
        let mut buffer: &[u8] = packet
            .payload
            .as_ref()
            .expect("Packet's payload field is empty");

        let Ok(rtcp_packets) = packet::unmarshal(&mut buffer) else {
            return None;
        };

        let packets: Vec<_> = rtcp_packets
            .into_iter()
            .map(|packet| Self::cast_to_packet(packet))
            .collect();

        Some(packets)
    }

    fn cast_to_packet(packet: Box<dyn rtcp::packet::Packet>) -> Self {
        // previously, I've used the for of rtcp library
        // but for the sake of being able to publish the crate on crates.io
        // I've reverted the changes, so some packets might not be handled properly
        use rtcp::goodbye::Goodbye;
        use rtcp::receiver_report::ReceiverReport;
        use rtcp::sender_report::SenderReport;
        use rtcp::source_description::SourceDescription;
        use rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
        use rtcp::payload_feedbacks::full_intra_request::FullIntraRequest;
        use rtcp::payload_feedbacks::receiver_estimated_maximum_bitrate::ReceiverEstimatedMaximumBitrate;
        use rtcp::payload_feedbacks::slice_loss_indication::SliceLossIndication;
        use rtcp::transport_feedbacks::rapid_resynchronization_request::RapidResynchronizationRequest;
        use rtcp::transport_feedbacks::transport_layer_cc::TransportLayerCc;
        use rtcp::transport_feedbacks::transport_layer_nack::TransportLayerNack;

        let packet_type = packet.header().packet_type;

        let packet = packet.as_any();
        match packet_type {
            PacketType::Unsupported => {
                return RtcpPacket::Other(packet_type as u8);
            }
            PacketType::SenderReport => {
                if let Some(pack) = packet.downcast_ref::<SenderReport>() {
                    return RtcpPacket::SenderReport(sender_report::SenderReport::new(pack));
                }
            }
            PacketType::ReceiverReport => {
                if let Some(pack) = packet.downcast_ref::<ReceiverReport>() {
                    return RtcpPacket::ReceiverReport(receiver_report::ReceiverReport::new(pack));
                }
            }
            PacketType::SourceDescription => {
                if let Some(pack) = packet.downcast_ref::<SourceDescription>() {
                    return RtcpPacket::SourceDescription(source_description::SourceDescription::new(pack));
                }
            }
            PacketType::Goodbye => {
                if let Some(pack) = packet.downcast_ref::<Goodbye>() {
                    return RtcpPacket::Goodbye(goodbye::Goodbye::new(pack));
                }
            }
            PacketType::ApplicationDefined => {
                return RtcpPacket::ApplicationDefined;
            }
            PacketType::TransportSpecificFeedback => {
                if let Some(_pack) = packet.downcast_ref::<RapidResynchronizationRequest>() {
                    return RtcpPacket::TransportSpecificFeedback(TransportFeedback::RapidResynchronizationRequest);
                }

                if let Some(_pack) = packet.downcast_ref::<TransportLayerCc>() {
                    return RtcpPacket::TransportSpecificFeedback(TransportFeedback::TransportLayerCc);
                }

                if let Some(_pack) = packet.downcast_ref::<TransportLayerNack>() {
                    return RtcpPacket::TransportSpecificFeedback(TransportFeedback::TransportLayerNack);
                }
            }
            PacketType::PayloadSpecificFeedback => {
                if let Some(pack) = packet.downcast_ref::<PictureLossIndication>() {
                    return RtcpPacket::PictureLossIndication(picture_loss_indication::PictureLossIndication::new(pack));
                }

                if let Some(pack) = packet.downcast_ref::<FullIntraRequest>() {
                    return RtcpPacket::FullIntraRequest(full_intra_request::FullIntraRequest::new(pack));
                }

                if let Some(pack) = packet.downcast_ref::<ReceiverEstimatedMaximumBitrate>() {
                    return RtcpPacket::ReceiverEstimatedMaximumBitrate(receiver_estimated_maximum_bitrate::ReceiverEstimatedMaximumBitrate::new(pack));
                }

                if let Some(pack) = packet.downcast_ref::<SliceLossIndication>() {
                    return RtcpPacket::SliceLossIndication(slice_loss_indication::SliceLossIndication::new(pack));
                }
            }
            PacketType::ExtendedReport => {
                return RtcpPacket::ExtendedReport;
            }
        }
        RtcpPacket::Other(packet_type as u8)
    }
}
