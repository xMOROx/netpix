use bincode::{Decode, Encode};
#[cfg(not(target_arch = "wasm32"))]
use std::any::Any;

#[derive(Decode, Encode, Debug, Clone)]
pub struct TransportFeedback {
    pub sender_ssrc: u32,
    pub media_ssrc: u32,
    pub feedback_type: TransportFeedbackType,
}

impl TransportFeedback {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(packet: &dyn Any) -> Option<TransportFeedback> {
        use rtcp::transport_feedbacks::rapid_resynchronization_request::RapidResynchronizationRequest;
        use rtcp::transport_feedbacks::transport_layer_cc::TransportLayerCc;
        use rtcp::transport_feedbacks::transport_layer_nack::TransportLayerNack;

        if let Some(pack) = packet.downcast_ref::<RapidResynchronizationRequest>() {
            return Some(TransportFeedback {
                sender_ssrc: pack.sender_ssrc,
                media_ssrc: pack.media_ssrc,
                feedback_type: TransportFeedbackType::RapidResynchronizationRequest,
            });
        }

        if let Some(pack) = packet.downcast_ref::<TransportLayerCc>() {
            return Some(TransportFeedback {
                sender_ssrc: pack.sender_ssrc,
                media_ssrc: pack.media_ssrc,
                feedback_type: TransportFeedbackType::TransportLayerCc,
            });
        }

        if let Some(pack) = packet.downcast_ref::<TransportLayerNack>() {
            return Some(TransportFeedback {
                sender_ssrc: pack.sender_ssrc,
                media_ssrc: pack.media_ssrc,
                feedback_type: TransportFeedbackType::TransportLayerNack,
            });
        }

        None
    }

    pub fn get_type_name(&self) -> &str {
        self.feedback_type.get_type_name()
    }
}

#[derive(Decode, Encode, Debug, Clone)]
pub enum TransportFeedbackType {
    TransportLayerCc,
    TransportLayerNack,
    RapidResynchronizationRequest,
}

impl TransportFeedbackType {
    pub fn get_type_name(&self) -> &str {
        match self {
            TransportFeedbackType::TransportLayerCc => "Transport Layer Cc",
            TransportFeedbackType::TransportLayerNack => "Transport Layer Nack",
            TransportFeedbackType::RapidResynchronizationRequest => {
                "Rapid Resynchronization Request"
            }
        }
    }
}
