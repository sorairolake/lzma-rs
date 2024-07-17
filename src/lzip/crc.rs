use crc::{Crc, CRC_32_ISO_HDLC};

pub const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
