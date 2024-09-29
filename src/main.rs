#![allow(unused)]

use bwt::BWT;
use clap::Parser;
use encoder::Encoder;
use rle::RLE;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    process::exit,
};

mod utils;
use utils::*;

mod encoders;
use encoders::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Reading file

    let mut input_file = File::open(&args.file)?;
    let mut buf: Vec<u8> = vec![];
    input_file.read_to_end(&mut buf);

    // Encoding

    let mut encoded = RLE::encode(buf)?;

    println!("{:?}", encoded);
    let mut output_file = File::create(format!("{}.pkzip", &args.file))?;
    output_file.write_all(&encoded);

    // Decoding

    let mut decoded = RLE::decode(encoded)?;
    decoded = BWT::decode(decoded)?;

    let mut decoded_file = File::create(format!("{}.decoded", &args.file))?;
    decoded_file.write_all(&decoded);

    Ok(())
}
