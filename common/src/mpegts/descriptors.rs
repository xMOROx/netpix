pub mod audio_stream;
pub mod avc_video;
pub mod conditional_access;
pub mod copyright;
pub mod data_stream_alignment;
pub mod hierarchy;
pub mod ibp;
pub mod iso_639_language;
pub mod macros;
pub mod maximum_bitrate;
pub mod mpeg4_audio;
pub mod mpeg4_video;
pub mod multiplex_buffer_utilization;
pub mod private_data_indicator;
pub mod registration;
pub mod smoothing_buffer;
pub mod std_descriptor;
pub mod system_clock;
pub mod tags;
pub mod target_background_grid;
pub mod video_stream;
pub mod video_window;

use crate::mpegts::descriptors::avc_video::AvcVideoDescriptor;
use crate::mpegts::descriptors::conditional_access::CaDescriptor;
use crate::mpegts::descriptors::copyright::CopyrightDescriptor;
use crate::mpegts::descriptors::data_stream_alignment::DataStreamAlignmentDescriptor;
use crate::mpegts::descriptors::hierarchy::HierarchyDescriptor;
use crate::mpegts::descriptors::ibp::IbpDescriptor;
use crate::mpegts::descriptors::iso_639_language::Iso639LanguageDescriptor;
use crate::mpegts::descriptors::maximum_bitrate::MaximumBitrateDescriptor;
use crate::mpegts::descriptors::mpeg4_audio::Mpeg4AudioDescriptor;
use crate::mpegts::descriptors::mpeg4_video::Mpeg4VideoDescriptor;
use crate::mpegts::descriptors::multiplex_buffer_utilization::MultiplexBufferUtilizationDescriptor;
use crate::mpegts::descriptors::private_data_indicator::PrivateDataIndicatorDescriptor;
use crate::mpegts::descriptors::registration::RegistrationDescriptor;
use crate::mpegts::descriptors::smoothing_buffer::SmoothingBufferDescriptor;
use crate::mpegts::descriptors::std_descriptor::StdDescriptor;
use crate::mpegts::descriptors::system_clock::SystemClockDescriptor;
use crate::mpegts::descriptors::tags::DescriptorTag;
use crate::mpegts::descriptors::target_background_grid::TargetBackgroundGridDescriptor;
use crate::mpegts::descriptors::video_stream::VideoStreamDescriptor;
use crate::mpegts::descriptors::video_window::VideoWindowDescriptor;
use crate::{
    declare_descriptor_variants, mpegts::descriptors::audio_stream::AudioStreamDescriptor,
};
use crate::{
    impl_descriptor_display, impl_descriptor_partial_eq, impl_descriptor_unmarshall_match,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

const HEADER_SIZE: u8 = 2;

pub trait ParsableDescriptor<T>: Debug {
    fn descriptor_tag(&self) -> u8;
    fn descriptor_length(&self) -> u8;
    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<T>;
}

declare_descriptor_variants! {
    (VideoStreamDescriptor, VideoStreamDescriptor),
    (AudioStreamDescriptor, AudioStreamDescriptor),
    (HierarchyDescriptor, HierarchyDescriptor),
    (RegistrationDescriptor, RegistrationDescriptor),
    (TargetBackgroundGridDescriptor, TargetBackgroundGridDescriptor),
    (VideoWindowDescriptor, VideoWindowDescriptor),
    (CaDescriptor, CaDescriptor),
    (SystemClockDescriptor, SystemClockDescriptor),
    (MaximumBitrateDescriptor, MaximumBitrateDescriptor),
    (CopyrightDescriptor, CopyrightDescriptor),
    (MultiplexBufferUtilizationDescriptor, MultiplexBufferUtilizationDescriptor),
    (PrivateDataIndicatorDescriptor, PrivateDataIndicatorDescriptor),
    (StdDescriptor, StdDescriptor),
    (DataStreamAlignmentDescriptor, DataStreamAlignmentDescriptor),
    (AvcVideoDescriptor, AvcVideoDescriptor),
    (Iso639LanguageDescriptor, Iso639LanguageDescriptor),
    (SmoothingBufferDescriptor, SmoothingBufferDescriptor),
    (IbpDescriptor, IbpDescriptor),
    (Mpeg4VideoDescriptor, Mpeg4VideoDescriptor),
    (Mpeg4AudioDescriptor, Mpeg4AudioDescriptor),
}

impl_descriptor_display! {
    (VideoStreamDescriptor),
    (AudioStreamDescriptor),
    (HierarchyDescriptor),
    (RegistrationDescriptor),
    (TargetBackgroundGridDescriptor),
    (VideoWindowDescriptor),
    (CaDescriptor),
    (SystemClockDescriptor),
    (MaximumBitrateDescriptor),
    (CopyrightDescriptor),
    (MultiplexBufferUtilizationDescriptor),
    (PrivateDataIndicatorDescriptor),
    (StdDescriptor),
    (DataStreamAlignmentDescriptor),
    (AvcVideoDescriptor),
    (Iso639LanguageDescriptor),
    (SmoothingBufferDescriptor),
    (IbpDescriptor),
    (Mpeg4VideoDescriptor),
    (Mpeg4AudioDescriptor),
}

impl_descriptor_unmarshall_match! {
    (VideoStreamDescriptor, VideoStreamDescriptorTag, VideoStreamDescriptor),
    (AudioStreamDescriptor, AudioStreamDescriptorTag, AudioStreamDescriptor),
    (HierarchyDescriptor, HierarchyDescriptorTag, HierarchyDescriptor),
    (RegistrationDescriptor, RegistrationDescriptorTag, RegistrationDescriptor),
    (TargetBackgroundGridDescriptor, TargetBackgroundGridDescriptorTag, TargetBackgroundGridDescriptor),
    (VideoWindowDescriptor, VideoWindowDescriptorTag, VideoWindowDescriptor),
    (CaDescriptor, CaDescriptorTag, CaDescriptor),
    (SystemClockDescriptor, SystemClockDescriptorTag, SystemClockDescriptor),
    (MaximumBitrateDescriptor, MaximumBitrateDescriptorTag, MaximumBitrateDescriptor),
    (CopyrightDescriptor, CopyrightDescriptorTag, CopyrightDescriptor),
    (MultiplexBufferUtilizationDescriptor, MultiplexBufferUtilizationDescriptorTag, MultiplexBufferUtilizationDescriptor),
    (PrivateDataIndicatorDescriptor, PrivateDataIndicatorDescriptorTag, PrivateDataIndicatorDescriptor),
    (StdDescriptor, StdDescriptorTag, StdDescriptor),
    (DataStreamAlignmentDescriptor, DataStreamAlignmentDescriptorTag, DataStreamAlignmentDescriptor),
    (AvcVideoDescriptor, AvcVideoDescriptorTag, AvcVideoDescriptor),
    (Iso639LanguageDescriptor, Iso639LanguageDescriptorTag, Iso639LanguageDescriptor),
    (SmoothingBufferDescriptor, SmoothingBufferDescriptorTag, SmoothingBufferDescriptor),
    (IbpDescriptor, IbpDescriptorTag, IbpDescriptor),
    (Mpeg4VideoDescriptor, Mpeg4VideoDescriptorTag, Mpeg4VideoDescriptor),
    (Mpeg4AudioDescriptor, Mpeg4AudioDescriptorTag, Mpeg4AudioDescriptor),
}

impl_descriptor_partial_eq! {
    (VideoStreamDescriptor),
    (AudioStreamDescriptor),
    (HierarchyDescriptor),
    (RegistrationDescriptor),
    (TargetBackgroundGridDescriptor),
    (VideoWindowDescriptor),
    (CaDescriptor),
    (SystemClockDescriptor),
    (MaximumBitrateDescriptor),
    (CopyrightDescriptor),
    (MultiplexBufferUtilizationDescriptor),
    (PrivateDataIndicatorDescriptor),
    (StdDescriptor),
    (DataStreamAlignmentDescriptor),
    (AvcVideoDescriptor),
    (Iso639LanguageDescriptor),
    (SmoothingBufferDescriptor),
    (IbpDescriptor),
    (Mpeg4VideoDescriptor),
    (Mpeg4AudioDescriptor),
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct DescriptorHeader {
    pub descriptor_tag: DescriptorTag,
    pub descriptor_length: u8,
}

impl DescriptorHeader {
    pub fn unmarshall(data: &[u8]) -> Self {
        let descriptor_tag = DescriptorTag::from(data[0]);
        let descriptor_length = data[1];

        DescriptorHeader {
            descriptor_tag,
            descriptor_length,
        }
    }
}

impl PartialEq for DescriptorHeader {
    fn eq(&self, other: &Self) -> bool {
        let descriptor_tag = self.descriptor_tag == other.descriptor_tag;
        let descriptor_length = self.descriptor_length == other.descriptor_length;

        descriptor_tag && descriptor_length
    }
}
