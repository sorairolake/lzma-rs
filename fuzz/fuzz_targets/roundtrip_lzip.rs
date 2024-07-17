#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use lzma_rs::error::Result;

fn round_trip_lzip(x: &[u8]) -> Result<Vec<u8>> {
    let mut compressed: Vec<u8> = Vec::new();
    lzma_rs::lzip_compress(&mut std::io::BufReader::new(x), &mut compressed)?;
    let mut bf = std::io::BufReader::new(compressed.as_slice());

    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::lzip_decompress(&mut bf, &mut decomp)?;
    Ok(decomp)
}

fuzz_target!(|data: &[u8]| {
    let decomp = round_trip_lzip(data).expect("Can't decompress what we just compressed");
    assert_eq!(decomp, data);
});
