pub trait DataParser {
    type Output;
    fn parse(data: &[u8]) -> Option<Self::Output>;
}

pub trait DataValidator {
    fn validate(&self) -> bool;
}

pub trait BitManipulation {
    fn get_bit(byte: u8, position: u8) -> bool {
        (byte & (1 << position)) != 0
    }

    fn get_bits(byte: u8, mask: u8, shift: u8) -> u8 {
        (byte & mask) >> shift
    }

    fn set_bit(byte: &mut u8, position: u8, value: bool) {
        if value {
            *byte |= 1 << position;
        } else {
            *byte &= !(1 << position);
        }
    }
}

pub trait DataAccumulator {
    fn accumulate_payload(&self) -> Vec<u8>;
    fn accumulate_descriptors(&self) -> Vec<u8>;
}
