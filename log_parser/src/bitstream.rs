//! Parser for bitstream data embedded in Chrome `rtc_event_log` entries.
//!
//! The `rtc_event_log` format is decoded with `protoc`, but certain fields
//! contain binary blobs that require additional parsing. This module provides
//! a Rust implementation of that parser, rewritten from the original
//! JavaScript version:
//! <https://github.com/fippo/dump-webrtc-event-log/blob/gh-pages/bitstream.js>

use std::io;

struct BitstreamReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitstreamReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        BitstreamReader {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    fn align_to_byte(&mut self) {
        if self.bit_pos != 0 {
            self.byte_pos += 1;
            self.bit_pos = 0;
        }
    }

    fn byte_offset(&self) -> usize {
        self.byte_pos
    }

    fn read_bits(&mut self, mut bits: u8) -> Result<u64, io::Error> {
        if bits > 64 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "max 64 bits"));
        }

        let total_bits = self.data.len() * 8;
        let consumed_bits = self.byte_pos * 8 + self.bit_pos as usize;
        if total_bits < consumed_bits + bits as usize {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of bitstream",
            ));
        }

        let mut result: u64 = 0;

        while bits > 0 {
            let avail = 8 - self.bit_pos;
            let take = std::cmp::min(bits, avail);

            let shift = avail - take;
            let mask = ((1u16 << take) - 1) as u8;
            let byte = self.data[self.byte_pos];
            let chunk = ((byte >> shift) & mask) as u64;

            result = (result << take) | chunk;

            self.bit_pos += take;
            if self.bit_pos == 8 {
                self.byte_pos += 1;
                self.bit_pos = 0;
            }

            bits -= take;
        }

        Ok(result)
    }
}

fn decode_var_int(reader: &mut BitstreamReader) -> Result<u64, std::io::Error> {
    let mut value: u64 = 0;
    for i in 0..10 {
        let byte = reader.read_bits(8)? as u8;
        value |= ((byte & 0x7F) as u64) << (7 * i);
        if (byte & 0x80) == 0 {
            return Ok(value);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "VarInt is too long or malformed",
    ))
}

#[derive(Debug, Clone)]
struct Params {
    delta_width_bits: u8,
    signed_deltas: bool,
    values_optional: bool,
    value_width_bits: u8,
}

pub struct FixedLengthDeltaDecoder<'a> {
    reader: Option<BitstreamReader<'a>>,
    base: u64,
    number_of_deltas: usize,
    params: Params,
}

impl<'a> FixedLengthDeltaDecoder<'a> {
    pub fn new(data: &'a [u8], base: u64, number_of_deltas: usize) -> Result<Self, std::io::Error> {
        if data.is_empty() {
            return Ok(Self {
                reader: None,
                base,
                number_of_deltas,
                params: Params {
                    delta_width_bits: 64,
                    signed_deltas: false,
                    values_optional: false,
                    value_width_bits: 64,
                },
            });
        }

        let mut reader = BitstreamReader::new(data);
        let encoding_type = reader.read_bits(2)?;

        let params = match encoding_type {
            0 => Params {
                delta_width_bits: reader.read_bits(6)? as u8 + 1,
                signed_deltas: false,
                values_optional: false,
                value_width_bits: 64,
            },
            1 => Params {
                delta_width_bits: reader.read_bits(6)? as u8 + 1,
                signed_deltas: reader.read_bits(1)? != 0,
                values_optional: reader.read_bits(1)? != 0,
                value_width_bits: reader.read_bits(6)? as u8 + 1,
            },
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unsupported encoding type",
                ));
            }
        };

        Ok(Self {
            reader: Some(reader),
            base,
            number_of_deltas,
            params,
        })
    }

    pub fn decode(&mut self) -> Result<Vec<Option<u64>>, std::io::Error> {
        if self.reader.is_none() {
            return Ok(vec![None; self.number_of_deltas]);
        }

        let delta_width = self.params.delta_width_bits;
        let num = self.number_of_deltas;

        let mut existing = vec![true; num];
        if self.params.values_optional {
            for i in 0..num {
                // borrow for the read, then drop it immediately
                let bit = self.reader.as_mut().unwrap().read_bits(1)?;
                existing[i] = bit != 0;
            }
        }

        let mut values = vec![None; num];
        let mut prev = self.base;
        for i in 0..num {
            if !existing[i] {
                continue;
            }

            let delta = self.reader.as_mut().unwrap().read_bits(delta_width)?;

            let cur = self.apply_delta(prev, delta);
            values[i] = Some(cur);
            prev = cur;
        }

        Ok(values)
    }

    fn apply_delta(&self, base: u64, delta: u64) -> u64 {
        let value = if self.params.signed_deltas {
            let top_bit = 1u64 << (self.params.delta_width_bits - 1);
            if (delta & top_bit) == 0 {
                base.wrapping_add(delta)
            } else {
                let mask = (1u64 << self.params.delta_width_bits) - 1;
                let delta_abs = (!delta & mask).wrapping_add(1);
                base.wrapping_sub(delta_abs)
            }
        } else {
            base.wrapping_add(delta)
        };
        if self.params.value_width_bits < 64 {
            let value_mask = u64::MAX >> (64 - self.params.value_width_bits);
            value & value_mask
        } else {
            value
        }
    }
}

pub struct BlobDecoder<'a> {
    data: &'a [u8],
    reader: Option<BitstreamReader<'a>>,
    number_of_deltas: usize,
}

impl<'a> BlobDecoder<'a> {
    pub fn new(data: &'a [u8], number_of_deltas: usize) -> Self {
        let reader = if data.is_empty() {
            None
        } else {
            Some(BitstreamReader::new(data))
        };
        Self {
            data,
            reader,
            number_of_deltas,
        }
    }

    pub fn decode(&mut self) -> Result<Vec<&'a [u8]>, std::io::Error> {
        let mut reader = match self.reader.as_mut() {
            Some(r) => r,
            None => return Ok(Vec::new()),
        };

        let mut lengths = Vec::with_capacity(self.number_of_deltas);
        for _ in 0..self.number_of_deltas {
            lengths.push(decode_var_int(&mut reader)? as usize);
        }

        reader.align_to_byte();
        let mut offset = reader.byte_offset();
        let mut values = Vec::with_capacity(self.number_of_deltas);
        for length in lengths {
            let end = offset + length;
            if end > self.data.len() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Blob length exceeds available data",
                ));
            }
            values.push(&self.data[offset..end]);
            offset = end;
        }

        Ok(values)
    }
}
