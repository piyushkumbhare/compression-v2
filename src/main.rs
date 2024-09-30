#![allow(unused)]

use bwt::Bwt;
use clap::Parser;
use encoder::Tokens;
use rle::Rle;
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

    let input_path = &args.file;
    let output_path = format!("{}.pkz", &args.file);
    let decoded_path = format!("{}.decoded", &args.file);

    // Reading file

    let mut input_file = File::open(&args.file)?;
    let mut buf: Vec<u8> = vec![];
    input_file.read_to_end(&mut buf);

    let input_size = buf.len();

    // Encoding

    let output = Tokens::new(buf)
        .encode_bwt()
        .encode_rle()
        .to_bytes();
    
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&output);

    let output_size = output.len();

    // Decoding

    let decoded = Tokens::new(output.clone())
        .decode_rle()
        .decode_bwt()
        .to_bytes();

    let mut decoded_file = File::create(&decoded_path)?;
    decoded_file.write_all(&decoded);

    let percent = (1.0 - output_size as f32 / input_size as f32) * 100.0;

    print!("Size (Bytes):");
    println!(
        r#"
{input_path} - {}
{output_path} - {}
Total compression: {:0.2}%

{decoded_path} - {}
    "#,
        get_file_size(&input_path)?,
        get_file_size(&output_path)?,
        percent,
        get_file_size(&decoded_path)?
    );

    Ok(())
}
