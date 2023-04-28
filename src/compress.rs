use base64::{engine::general_purpose, Engine as _};
use bytes::BytesMut;
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::io::{Read, Write};

pub fn compress(input: &str) -> String {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    general_purpose::STANDARD.encode(compressed_data)
}
pub fn decompress(compressed_data: &str) -> String {
    let decoded_data = general_purpose::STANDARD.decode(compressed_data).unwrap();
    let mut decoder = DeflateDecoder::new(&decoded_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();
    String::from_utf8(decompressed_data).unwrap()
}
