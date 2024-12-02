pub mod audio_stream;
pub mod avc_video_descriptor;
pub mod ca_descriptor;
pub mod copyright_descriptor;
pub mod data_stream_alignment_descriptor;
pub mod hierarchy;
pub mod iso_639_language_descriptor;
pub mod macros;
pub mod maximum_bitrate_descriptor;
pub mod multiplex_buffer_utilization_descriptor;
pub mod private_data_indicator_descriptor;
pub mod registration_descriptor;
pub mod std_descriptor;
pub mod system_clock_descriptor;
pub mod tags;
pub mod target_background_grid_descriptor;
pub mod video_stream;
pub mod video_window_descriptor;

use crate::mpegts::descriptors::avc_video_descriptor::AvcVideoDescriptor;
use crate::mpegts::descriptors::ca_descriptor::CaDescriptor;
use crate::mpegts::descriptors::copyright_descriptor::CopyrightDescriptor;
use crate::mpegts::descriptors::data_stream_alignment_descriptor::DataStreamAlignmentDescriptor;
use crate::mpegts::descriptors::hierarchy::HierarchyDescriptor;
use crate::mpegts::descriptors::iso_639_language_descriptor::Iso639LanguageDescriptor;
use crate::mpegts::descriptors::maximum_bitrate_descriptor::MaximumBitrateDescriptor;
use crate::mpegts::descriptors::multiplex_buffer_utilization_descriptor::MultiplexBufferUtilizationDescriptor;
use crate::mpegts::descriptors::private_data_indicator_descriptor::PrivateDataIndicatorDescriptor;
use crate::mpegts::descriptors::registration_descriptor::RegistrationDescriptor;
use crate::mpegts::descriptors::std_descriptor::StdDescriptor;
use crate::mpegts::descriptors::system_clock_descriptor::SystemClockDescriptor;
use crate::mpegts::descriptors::tags::DescriptorTag;
use crate::mpegts::descriptors::target_background_grid_descriptor::TargetBackgroundGridDescriptor;
use crate::mpegts::descriptors::video_stream::VideoStreamDescriptor;
use crate::mpegts::descriptors::video_window_descriptor::VideoWindowDescriptor;
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
