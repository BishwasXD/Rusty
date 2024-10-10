extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::process::Output;
use std::time::Instant;
// https://crates.io/crates/flate2
// compresses the file but may be unreadable,according to docs supports deglate ,zlib and gzip
// files
pub fn compression() {
    if args().len() != 3 {
        eprintln!("Usage:`source``target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Source len:{:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len:{:?}", output.metadata().unwrap().len());
}
