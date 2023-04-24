use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{self, Cursor, Read, Write};

pub fn compresse(v: &str) -> String {
    let val = v.as_bytes();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(val).unwrap();
    let compressed_val = encoder.finish().unwrap();
    base64::encode(&compressed_val)
}

pub fn decompress(v: String) -> String {
    let compressed_val = base64::decode(v).unwrap();
    let mut decoder = ZlibDecoder::new(Cursor::new(&compressed_val));
    let mut decompressed_bytes = Vec::new();
    decoder.read_to_end(&mut decompressed_bytes).unwrap();
    String::from_utf8(decompressed_bytes).unwrap()
}
