use std::io;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::prelude::*;

pub(super) fn deflate(uml: String) -> io::Result<Vec<u8>> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(uml.as_bytes())?;
    encoder.finish()
}