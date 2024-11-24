pub mod bits;
pub mod bytes;
pub mod crc;
pub mod traits;

// Re-export commonly used items
pub use bits::BitReader;
pub use bytes::ByteOperations;
pub use crc::Crc32Reader;
pub use traits::{BitManipulation, DataAccumulator, DataParser, DataValidator};
