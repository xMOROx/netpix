#[macro_export]
macro_rules! declare_descriptor_variants {
    ($(($variant:ident, $type:ty)),* $(,)?) => {
        #[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
        pub enum Descriptors {
            $($variant($type),)*
            UserPrivate(u8),
            Unknown,
        }
    };
}

#[macro_export]
macro_rules! impl_descriptor_display {
    ($(($variant:ident)),* $(,)?) => {
        impl std::fmt::Display for Descriptors {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Descriptors::$variant(descriptor) => write!(f, "{}", descriptor),)*
                    Descriptors::UserPrivate(data) => write!(f, "User Private: {}", data),
                    Descriptors::Unknown => write!(f, "Unknown"),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_descriptor_partial_eq {
    ($(($variant:ident)),* $(,)?) => {
        impl PartialEq for Descriptors {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (Descriptors::$variant(a), Descriptors::$variant(b)) => a == b,
                    )*
                    (Descriptors::UserPrivate(a), Descriptors::UserPrivate(b)) => a == b,
                    (Descriptors::Unknown, Descriptors::Unknown) => true,
                    _ => false,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_descriptor_unmarshall_match {
    ($(($variant:ident, $tag:ident, $type:ty)),* $(,)?) => {
        impl Descriptors {
            pub fn unmarshall(data: &[u8]) -> Option<Self> {
                let header = DescriptorHeader::unmarshall(data);
                let payload = &data[2..];
                match header.descriptor_tag {
                    $(
                        DescriptorTag::$tag => {
                            <$type>::unmarshall(header, payload)
                                .map(Descriptors::$variant)
                        }
                    )*
                    DescriptorTag::UserPrivate => Some(Descriptors::UserPrivate(data[0])),
                    _ => Some(Descriptors::Unknown),
                }
            }
            pub fn unmarshall_many(data: &[u8]) -> Vec<Self> {
                let mut descriptors = Vec::new();
                let mut offset = 0;
                while offset < data.len() {
                    let header = DescriptorHeader::unmarshall(&data[offset..]);
                    if let Some(descriptor) = Self::unmarshall(
                        &data[offset..(header.descriptor_length + HEADER_SIZE) as usize + offset]
                    ) {
                        descriptors.push(descriptor);
                    }
                    offset += header.descriptor_length as usize + HEADER_SIZE as usize;
                }
                descriptors
            }
        }
    };
}

#[macro_export]
macro_rules! implement_descriptor {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $type:ty
            ),* $(,)?
        }
        unmarshall_impl: |$header:ident, $data:ident| $unmarshall:block
        $(,)?
    ) => {
        $(#[$struct_meta])*
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
        $vis struct $name {
            pub header: $crate::mpegts::descriptors::DescriptorHeader,
            $(
                $(#[$field_meta])*
                $field_vis $field: $type
            ),*
        }

        impl $crate::mpegts::descriptors::ParsableDescriptor<$name> for $name {
            fn descriptor_tag(&self) -> u8 {
                self.header.descriptor_tag.to_u8()
            }

            fn descriptor_length(&self) -> u8 {
                self.header.descriptor_length
            }

            fn unmarshall($header: $crate::mpegts::descriptors::DescriptorHeader, $data: &[u8]) -> Option<$name> {
                $unmarshall
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!($name)
                    .chars()
                    .fold(String::new(), |mut acc, c| {
                        if c.is_uppercase() && !acc.is_empty() {
                            acc.push(' ');
                        }
                        acc.push(c);
                        acc
                    }))?;
                $(
                    write!(f, "\n{}: {:?}",
                        stringify!($field)
                            .replace("_", " ")
                            .split_whitespace()
                            .map(|word| {
                                let mut chars = word.chars();
                                match chars.next() {
                                    None => String::new(),
                                    Some(first) => first.to_uppercase().chain(chars).collect()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                        self.$field)?;
                )*
                write!(f, "\n")
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.header == other.header
                $(
                    && self.$field == other.$field
                )*
            }
        }
    };
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $type:ty
            ),* $(,)?
        }
        unmarshall_impl: |$header:ident, $data:ident| $unmarshall:block
        ;
        custom_display: $display:item
        $(,)?
    ) => {
        $(#[$struct_meta])*
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
        $vis struct $name {
            pub header: $crate::mpegts::descriptors::DescriptorHeader,
            $(
                $(#[$field_meta])*
                $field_vis $field: $type
            ),*
        }

        impl $crate::mpegts::descriptors::ParsableDescriptor<$name> for $name {
            fn descriptor_tag(&self) -> u8 {
                self.header.descriptor_tag.to_u8()
            }

            fn descriptor_length(&self) -> u8 {
                self.header.descriptor_length
            }

            fn unmarshall($header: $crate::mpegts::descriptors::DescriptorHeader, $data: &[u8]) -> Option<$name> {
                $unmarshall
            }
        }

        $display

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.header == other.header
                $(
                    && self.$field == other.$field
                )*
            }
        }
    };
}
