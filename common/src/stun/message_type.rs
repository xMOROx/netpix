use super::class::MessageClass;
use super::method::Method;
use bincode::{Decode, Encode};

const METHOD_ABITS: u16 = 0xf; // 0b0000000000001111
const METHOD_BBITS: u16 = 0x70; // 0b0000000001110000
const METHOD_DBITS: u16 = 0xf80; // 0b0000111110000000

const METHOD_BSHIFT: u16 = 1;
const METHOD_DSHIFT: u16 = 2;

const FIRST_BIT: u16 = 0x1;
const SECOND_BIT: u16 = 0x2;

const C0BIT: u16 = FIRST_BIT;
const C1BIT: u16 = SECOND_BIT;

const CLASS_C0SHIFT: u16 = 4;
const CLASS_C1SHIFT: u16 = 7;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Encode, Decode)]
pub struct MessageType {
    pub method: Method,      // e.g. binding
    pub class: MessageClass, // e.g. request
}

impl MessageType {
    // ReadValue decodes uint16 into MessageType.
    pub fn new(value: u16) -> Self {
        // Decoding class.
        // We are taking first bit from v >> 4 and second from v >> 7.
        let c0 = (value >> CLASS_C0SHIFT) & C0BIT;
        let c1 = (value >> CLASS_C1SHIFT) & C1BIT;
        let class = c0 + c1;
        let class = MessageClass(class as u8);

        // Decoding method.
        let a = value & METHOD_ABITS; // A(M0-M3)
        let b = (value >> METHOD_BSHIFT) & METHOD_BBITS; // B(M4-M6)
        let d = (value >> METHOD_DSHIFT) & METHOD_DBITS; // D(M7-M11)
        let m = a + b + d;
        let method = Method(m);

        MessageType { method, class }
    }

    pub fn as_string(&self) -> String {
        format!("{} {}", self.class.as_string(), self.method.as_string())
    }
}
