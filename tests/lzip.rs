#[cfg(feature = "enable_logging")]
use log::{debug, info};
use std::io::{BufReader, Read};

/// Utility function to read a file into memory
fn read_all_file(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    std::fs::File::open(filename).and_then(|mut file| file.read_to_end(&mut data))?;
    Ok(data)
}

fn round_trip(x: &[u8]) {
    let mut compressed: Vec<u8> = Vec::new();
    lzma_rs::lzip_compress(&mut std::io::BufReader::new(x), &mut compressed).unwrap();
    #[cfg(feature = "enable_logging")]
    info!("Compressed {} -> {} bytes", x.len(), compressed.len());
    #[cfg(feature = "enable_logging")]
    debug!("Compressed content: {:?}", compressed);
    let mut bf = BufReader::new(compressed.as_slice());
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::lzip_decompress(&mut bf, &mut decomp).unwrap();
    assert_eq!(decomp, x)
}

fn round_trip_file(filename: &str) {
    let x = read_all_file(filename).unwrap();
    round_trip(x.as_slice());
}

#[test]
fn round_trip_basics() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    round_trip(b"");
    // Note: we use vec! to avoid storing the slice in the binary
    round_trip(vec![0x00; 1_000_000].as_slice());
    round_trip(vec![0xFF; 1_000_000].as_slice());
}

#[test]
fn round_trip_hello() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    round_trip(b"Hello world");
}

#[test]
fn round_trip_files() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    round_trip_file("tests/files/foo.txt");
}

fn decomp_big_file(compfile: &str, plainfile: &str) {
    let expected = read_all_file(plainfile).unwrap();
    let mut f = BufReader::new(std::fs::File::open(compfile).unwrap());
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::lzip_decompress(&mut f, &mut decomp).unwrap();
    assert!(decomp == expected)
}

#[test]
fn big_file() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    decomp_big_file("tests/files/foo.txt.lz", "tests/files/foo.txt");
    decomp_big_file(
        "tests/files/good-1-lzma2-1.lz",
        "tests/files/good-1-lzma2-1",
    );
    decomp_big_file(
        "tests/files/good-1-lzma2-2.lz",
        "tests/files/good-1-lzma2-2",
    );
    decomp_big_file(
        "tests/files/good-1-lzma2-3.lz",
        "tests/files/good-1-lzma2-3",
    );
    decomp_big_file(
        "tests/files/good-1-lzma2-4.lz",
        "tests/files/good-1-lzma2-4",
    );
}

#[test]
fn decompress_empty_world() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    let mut x: &[u8] = b"\x4c\x5a\x49\x50\x01\x0c\x00\x83\xff\xfb\xff\xff\xc0\x00\x00\x00\
                         \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x24\x00\x00\x00\
                         \x00\x00\x00\x00";
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::lzip_decompress(&mut x, &mut decomp).unwrap();
    assert_eq!(decomp, b"")
}

#[test]
fn decompress_hello_world() {
    #[cfg(feature = "enable_logging")]
    let _ = env_logger::try_init();
    let mut x: &[u8] = b"\x4c\x5a\x49\x50\x01\x0c\x00\x24\x19\x49\x98\x6f\x10\x19\xc6\xd7\
                         \x31\xeb\x36\x50\xb2\x98\x48\xff\xfe\xa5\xb0\x00\xd5\xe0\x39\xb7\
                         \x0c\x00\x00\x00\x00\x00\x00\x00\x30\x00\x00\x00\x00\x00\x00\x00";
    let mut decomp: Vec<u8> = Vec::new();
    lzma_rs::lzip_decompress(&mut x, &mut decomp).unwrap();
    assert_eq!(decomp, b"Hello world\x0a")
}
