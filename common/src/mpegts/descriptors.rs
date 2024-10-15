pub mod tags;
pub mod video_stream;
pub mod audio_stream;
pub mod hierarchy;
pub mod maximum_bitrate_descriptor;
pub mod multiplex_buffer_utilization_descriptor;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::audio_stream::AudioStreamDescriptor;
use crate::mpegts::descriptors::hierarchy::HierarchyDescriptor;
use crate::mpegts::descriptors::maximum_bitrate_descriptor::MaximumBitrateDescriptor;
use crate::mpegts::descriptors::multiplex_buffer_utilization_descriptor::MultiplexBufferUtilizationDescriptor;
use crate::mpegts::descriptors::tags::DescriptorTag;
use crate::mpegts::descriptors::video_stream::VideoStreamDescriptor;

#[cfg(not(target_arch = "wasm32"))]
const HEADER_SIZE: u8 = 2;

pub trait ParsableDescriptor<T>: Debug {
    fn descriptor_tag(&self) -> u8;
    fn descriptor_length(&self) -> u8;
    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<T>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub enum Descriptors {
    VideoStreamDescriptor(VideoStreamDescriptor),
    AudioStreamDescriptor(AudioStreamDescriptor),
    HierarchyDescriptor(HierarchyDescriptor),
    MaximumBitrateDescriptor(MaximumBitrateDescriptor),
    MultiplexBufferUtilizationDescriptor(MultiplexBufferUtilizationDescriptor),
}

impl Descriptors {
    pub fn unmarshall(data: &[u8]) -> Option<Self> {
        let header = DescriptorHeader::unmarshall(data);
        let payload = &data[2..];
        match header.descriptor_tag {
            DescriptorTag::VideoStreamDescriptorTag => {
                VideoStreamDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptors::VideoStreamDescriptor(descriptor)
                })
            }
            DescriptorTag::AudioStreamDescriptorTag => {
                AudioStreamDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptors::AudioStreamDescriptor(descriptor)
                })
            }
            DescriptorTag::HierarchyDescriptorTag => {
                HierarchyDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptors::HierarchyDescriptor(descriptor)
                })
            }
            DescriptorTag::MaximumBitrateDescriptorTag => {
                MaximumBitrateDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptors::MaximumBitrateDescriptor(descriptor)
                })
            }
            DescriptorTag::MultiplexBufferUtilizationDescriptorTag => {
                MultiplexBufferUtilizationDescriptor::unmarshall(header, payload).map(|descriptor| {
                    Descriptors::MultiplexBufferUtilizationDescriptor(descriptor)
                })
            }
            _ => None,
        }
    }
    pub fn unmarshall_many(data: &[u8]) -> Vec<Self> {
        let mut descriptors = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            let header = DescriptorHeader::unmarshall(&data[offset..]);
            Self::unmarshall(&data[offset..(header.descriptor_length + HEADER_SIZE) as usize + offset]).map(|descriptor| {
                descriptors.push(descriptor);
            });
            offset += (HEADER_SIZE + header.descriptor_length) as usize;

        }
        descriptors
    }
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

impl PartialEq for Descriptors {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq for DescriptorHeader {
    fn eq(&self, other: &Self) -> bool {
        let descriptor_tag = self.descriptor_tag == other.descriptor_tag;
        let descriptor_length = self.descriptor_length == other.descriptor_length;

        descriptor_tag && descriptor_length
    }
}