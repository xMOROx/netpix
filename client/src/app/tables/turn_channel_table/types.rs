use crate::define_filter_context;

define_filter_context!(TurnChannelFilterContext,
    channel_number: u16,
    source_addr: str,
    destination_addr: str
);
