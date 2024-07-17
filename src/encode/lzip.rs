use crate::lzip::crc::CRC32;
use crate::lzip::header;
use crate::lzma_compress;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io;

pub fn encode_stream<R, W>(input: &mut R, output: &mut W) -> io::Result<()>
where
    R: io::BufRead,
    W: io::Write,
{
    // Header
    write_header(output)?;

    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    // Raw LZMA stream
    let compressed_data_size = write_stream(&buf, output)?;

    // Footer
    write_footer(&buf, output, compressed_data_size)
}

fn write_header<W>(output: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    output.write_all(header::LZIP_MAGIC)?;
    output.write_u8(header::LZIP_VERSION_NUMBER)?;
    // Pre-computed coded dictionary size which represents 0x00800000 (8388608 in
    // decimal). TODO: It is recommended that this be fixed when it becomes
    // possible to specify the dictionary size when compressing.
    let dict_size = 0x17;
    output.write_u8(dict_size)
}

fn write_footer<W>(input: &[u8], output: &mut W, compressed_data_size: u64) -> io::Result<()>
where
    W: io::Write,
{
    let digest = CRC32.checksum(input);
    output.write_u32::<LittleEndian>(digest)?;
    output.write_u64::<LittleEndian>(input.len() as u64)?;
    // header_size + compressed_data_size + footer_size
    output.write_u64::<LittleEndian>(6 + compressed_data_size + 20)
}

fn write_stream<W>(mut input: &[u8], output: &mut W) -> io::Result<u64>
where
    W: io::Write,
{
    let mut buf = Vec::new();
    lzma_compress(&mut input, &mut buf)?;
    // Drop the LZMA header.
    // TODO: It is recommended that this be fixed when it becomes possible to
    // generate the LZMA stream without the header.
    buf = buf.split_off(13);
    output.write_all(&buf)?;
    Ok(buf.len() as u64)
}
