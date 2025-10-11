use std::io;

// A helper struct to read bits from a byte slice.
// This is a more idiomatic Rust implementation than a direct port of the JS version.
struct BitstreamReader<'a> {
    data: &'a [u8],
    byte_pos: usize, // index of current byte
    bit_pos: u8,     // number of bits already consumed from current byte (0..8), counted from MSB
}

impl<'a> BitstreamReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        BitstreamReader { data, byte_pos: 0, bit_pos: 0 }
    }

    /// Align to next byte boundary (if we're mid-byte, skip remaining bits of that byte).
    fn align_to_byte(&mut self) {
        if self.bit_pos != 0 {
            self.byte_pos += 1;
            self.bit_pos = 0;
        }
    }

    fn byte_offset(&self) -> usize {
        self.byte_pos
    }

    /// Read up to 64 bits, MSB-first
    fn read_bits(&mut self, mut bits: u8) -> Result<u64, io::Error> {
        if bits > 64 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "max 64 bits"));
        }

        let total_bits = self.data.len() * 8;
        let consumed_bits = self.byte_pos * 8 + self.bit_pos as usize;
        if total_bits < consumed_bits + bits as usize {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of bitstream"));
        }

        let mut result: u64 = 0;

        while bits > 0 {
            let avail = 8 - self.bit_pos; // bits available in current byte (from MSB side)
            let take = std::cmp::min(bits, avail);

            // We want the top `take` bits from the remaining part of the current byte.
            // shift = how many LSBs to discard to align those bits to the LSB.
            let shift = avail - take;
            let mask = ((1u16 << take) - 1) as u8;
            let byte = self.data[self.byte_pos];
            let chunk = ((byte >> shift) & mask) as u64;

            // Append chunk to the right of previously-read bits, preserving MSB-first assembly:
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

/// Decodes a VarInt from the bitstream.
fn decode_var_int(reader: &mut BitstreamReader) -> Result<u64, std::io::Error> {
    let mut value: u64 = 0;
    for i in 0..10 {
        let byte = reader.read_bits(8)? as u8;
        // The lower 7 bits of each byte are part of the value.
        value |= ((byte & 0x7F) as u64) << (7 * i);
        // The MSB indicates if more bytes follow. If it's 0, we're done.
        if (byte & 0x80) == 0 {
            return Ok(value);
        }
    }
    // If the loop completes, the VarInt is malformed (too long).
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "VarInt is too long or malformed",
    ))
}


/// Parameters for the delta decoding process, parsed from the stream header.
#[derive(Debug, Clone)]
struct Params {
    delta_width_bits: u8,
    signed_deltas: bool,
    values_optional: bool,
    value_width_bits: u8,
}

/// Decodes a delta-encoded sequence of values where deltas have a fixed bit-width.
pub struct FixedLengthDeltaDecoder<'a> {
    reader: Option<BitstreamReader<'a>>,
    base: u64,
    number_of_deltas: usize,
    pub(crate) params: Params,
}

impl<'a> FixedLengthDeltaDecoder<'a> {
    /// Creates a new decoder and parses the header from the data stream.
    pub fn new(data: &'a [u8], base: u64, number_of_deltas: usize) -> Result<Self, std::io::Error> {
        // If data is empty, we can't decode anything. The behavior is to return
        // an array of the base value, which is handled in `decode`.
        if data.is_empty() {
            return Ok(Self {
                reader: None,
                base,
                number_of_deltas,
                // Default parameters when no data is present. These won't be used.
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

    /// Decodes the sequence of values.
    /// Returns a vector of `Option<u64>`, where `None` represents a missing value.
    pub fn decode(&mut self) -> Result<Vec<Option<u64>>, std::io::Error> {

        if self.reader.is_none() {
            return Ok(vec![None; self.number_of_deltas]);
        }

        // Pre-pull these so we don't re-borrow `self.params` inside the loop:
        let delta_width = self.params.delta_width_bits;
        let num = self.number_of_deltas;

        // 1) Determine which slots actually have values
        let mut existing = vec![true; num];
        if self.params.values_optional {
            for i in 0..num {
                // borrow for the read, then drop it immediately
                let bit = self.reader
                    .as_mut().unwrap()
                    .read_bits(1)?;
                existing[i] = bit != 0;
            }
        }

        // 2) Decode each delta
        let mut values = vec![None; num];
        let mut prev = self.base;
        for i in 0..num {
            if !existing[i] {
                continue;
            }

            // borrow reader just for this one read, then drop it
            let delta = self.reader
                .as_mut().unwrap()
                .read_bits(delta_width)?;

            // now that the mutable borrow is gone, we can call into &self
            let cur = self.apply_delta(prev, delta);
            values[i] = Some(cur);
            prev = cur;
        }

        Ok(values)
    }


    /// Applies a delta to a base value according to the decoder's parameters.
    fn apply_delta(&self, base: u64, delta: u64) -> u64 {
        let value = if self.params.signed_deltas {
            let top_bit = 1u64 << (self.params.delta_width_bits - 1);
            if (delta & top_bit) == 0 {
                // Positive delta: use wrapping_add for overflow behavior consistent with C++.
                base.wrapping_add(delta)
            } else {
                // Negative delta. To get the absolute value, we perform two's complement.
                let mask = (1u64 << self.params.delta_width_bits) - 1;
                let delta_abs = (!delta & mask).wrapping_add(1);
                base.wrapping_sub(delta_abs)
            }
        } else {
            // Unsigned delta.
            base.wrapping_add(delta)
        };

        // Truncate the value to the specified `value_width_bits`.
        if self.params.value_width_bits < 64 {
            let value_mask = u64::MAX >> (64 - self.params.value_width_bits);
            value & value_mask
        } else {
            value
        }
    }
}


/// Decodes a sequence of binary blobs (byte arrays).
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

    /// Decodes the blobs. First, all lengths are decoded as VarInts from the
    /// bitstream. Then, the rest of the raw byte data is sliced up according
    /// to those lengths.
    pub fn decode(&mut self) -> Result<Vec<&'a [u8]>, std::io::Error> {
        let mut reader = match self.reader.as_mut() {
            Some(r) => r,
            None => return Ok(Vec::new()),
        };

        // 1. Decode all blob lengths using VarInt.
        let mut lengths = Vec::with_capacity(self.number_of_deltas);
        for _ in 0..self.number_of_deltas {
            lengths.push(decode_var_int(&mut reader)? as usize);
        }

        // 2. The bitstream reading is done. Align to the next byte to find the start of raw blob data.
        reader.align_to_byte();
        let mut offset = reader.byte_offset();

        // 3. Slice the original data buffer to get each blob.
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