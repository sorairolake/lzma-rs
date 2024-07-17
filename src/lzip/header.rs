//! lzip header.

use crate::error;

/// File format magic header signature.
pub(crate) const LZIP_MAGIC: &[u8] = b"LZIP";

/// File format version number, 1 for now.
pub(crate) const LZIP_VERSION_NUMBER: u8 = 1;

/// lzip header.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Header {
    pub(crate) dict_size: u32,
}

impl Header {
    /// Parses the lzip header from a buffered reader.
    pub(crate) fn parse(input: [u8; 6]) -> error::Result<Self> {
        if &input[..4] != LZIP_MAGIC {
            return Err(error::Error::LzipError(format!(
                "Invalid lzip magic, expected {:?}",
                LZIP_MAGIC
            )));
        }

        match input[4] {
            LZIP_VERSION_NUMBER => {}
            0 => {
                return Err(error::Error::LzipError(String::from(
                    "lzip version 0 is not supported",
                )));
            }
            _ => {
                return Err(error::Error::LzipError(format!(
                    "Unknown lzip version number, expected {:?}",
                    LZIP_VERSION_NUMBER
                )));
            }
        }

        let mut dict_size = 1 << (input[5] & 0x1f);
        dict_size -= (dict_size / 16) * ((input[5] >> 5) & 0x07) as u32;
        match dict_size {
            ds if ds < (1 << 12) => {
                return Err(error::Error::LzipError(String::from(
                    "dictionary size too small, must be at least 4 KiB",
                )));
            }
            ds if ds > (1 << 29) => {
                return Err(error::Error::LzipError(String::from(
                    "dictionary size too large, must be less than 512 MiB",
                )));
            }
            _ => {}
        }
        let header = Self { dict_size };
        Ok(header)
    }
}
