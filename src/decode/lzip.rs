//! Decoder for the `.lz` file format.

use crate::decode::lzma::{LzmaDecoder, LzmaParams, LzmaProperties};
use crate::error;
use crate::lzip::crc::CRC32;
use crate::lzip::header;
use byteorder::{ByteOrder, LittleEndian};
use std::io;

pub fn decode_stream<R, W>(input: &mut R, output: &mut W) -> error::Result<()>
where
    R: io::BufRead,
    W: io::Write,
{
    let mut header_buf = [0; 6];
    input.read_exact(&mut header_buf)?;
    let header = header::Header::parse(header_buf)?;
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    let footer = buf.split_off(buf.len() - 20);
    let unpacked_size = LittleEndian::read_u64(&footer[4..12]);
    // See <https://www.nongnu.org/lzip/manual/lzip_manual.html#Stream-format>.
    let properties = LzmaProperties {
        lc: 3,
        lp: 0,
        pb: 2,
    };
    let params = LzmaParams::new(properties, header.dict_size, Some(unpacked_size));
    let mut uncompressed_data = Vec::new();
    LzmaDecoder::new(params, None)?.decompress(&mut buf.as_slice(), &mut uncompressed_data)?;

    let crc32 = CRC32.checksum(&uncompressed_data);
    let expected_crc32 = LittleEndian::read_u32(&footer[..4]);
    if crc32 != expected_crc32 {
        return Err(error::Error::LzipError(format!(
            "Invalid uncompressed data CRC32: expected 0x{:08x} but got 0x{:08x}",
            expected_crc32, crc32
        )));
    }

    if uncompressed_data.len() as u64 != unpacked_size {
        return Err(error::Error::LzipError(format!(
            "Invalid uncompressed data size: expected {} but got {}",
            unpacked_size,
            uncompressed_data.len()
        )));
    }

    let member_size = (header_buf.len() + buf.len() + footer.len()) as u64;
    let expected_member_size = LittleEndian::read_u64(&footer[12..]);
    if member_size > (1 << 51) {
        return Err(error::Error::LzipError(String::from(
            "member size too large, must be less than 2 PiB",
        )));
    }
    if member_size != expected_member_size {
        return Err(error::Error::LzipError(format!(
            "Invalid member size: expected {} but got {}",
            expected_member_size, member_size
        )));
    }
    output.write_all(&uncompressed_data)?;
    Ok(())
}
