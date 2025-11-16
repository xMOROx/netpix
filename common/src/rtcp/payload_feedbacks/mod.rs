pub mod fir_entry;
pub mod full_intra_request;
pub mod picture_loss_indication;
pub mod receiver_estimated_maximum_bitrate;
pub mod sli_entry;
pub mod slice_loss_indication;

pub use fir_entry::FirEntry;
pub use full_intra_request::FullIntraRequest;
pub use picture_loss_indication::PictureLossIndication;
pub use receiver_estimated_maximum_bitrate::ReceiverEstimatedMaximumBitrate;
pub use sli_entry::SliEntry;
pub use slice_loss_indication::SliceLossIndication;

use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub enum PayloadFeedback {
    PictureLossIndication(PictureLossIndication),
    FullIntraRequest(FullIntraRequest),
    ReceiverEstimatedMaximumBitrate(ReceiverEstimatedMaximumBitrate),
    SliceLossIndication(SliceLossIndication),
}

impl PayloadFeedback {
    pub fn get_type_name(&self) -> &str {
        match self {
            PayloadFeedback::PictureLossIndication(_) => "Picture Loss Indication",
            PayloadFeedback::ReceiverEstimatedMaximumBitrate(_) => {
                "Receiver Estimated Maximum Bitrate"
            }
            PayloadFeedback::SliceLossIndication(_) => "Slice Loss Indication",
            PayloadFeedback::FullIntraRequest(_) => "Full Intra Request",
        }
    }
}
