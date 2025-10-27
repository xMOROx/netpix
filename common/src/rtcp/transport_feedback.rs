use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub enum TransportFeedback {
    TransportLayerCc,
    TransportLayerNack,
    RapidResynchronizationRequest,
}

impl TransportFeedback {
    pub fn get_type_name(&self) -> &str {
        match self {
            TransportFeedback::TransportLayerCc => "Transport LayerCc",
            TransportFeedback::TransportLayerNack => "Transport Layer Nack",
            TransportFeedback::RapidResynchronizationRequest => "Rapid Resynchronization Request"
        }
    }
}
