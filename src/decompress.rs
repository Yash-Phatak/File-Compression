use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, BufWriter, copy};
use std::time::Instant;

pub fn decompress(source: &str, target: &str) {
    let start = Instant::now();

    let input = File::open(source).expect("Failed to open compressed file");
    let mut decoder = GzDecoder::new(BufReader::new(input));
    let output = File::create(target).expect("Failed to create output file");
    let mut writer = BufWriter::new(output);

    match copy(&mut decoder, &mut writer) {
        Ok(bytes) => println!("Decompressed {} bytes in {:?}", bytes, start.elapsed()),
        Err(e) => eprintln!("Failed to decompress data: {}", e),
    }
}
