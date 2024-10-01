// #![allow(unused)]
mod utils;
mod encoders;

use encoder::{Compress, Encoder};
use utils::*;
use encoders::*;

use std::error::Error;

use clap::Parser;
use colored::Colorize;
use sha256::digest;


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input_path = &args.file;
    let output_path = format!("{}.pkz", &args.file);
    let decoded_path = format!("{}.decoded", &args.file);

    // Reading file

    let buf = std::fs::read(input_path)?;

    let original_sha256 = digest(&buf);

    let input_size = buf.len();

    // Encoding

    let mut compressor = Compress::new(buf, &[Encoder::Rle]);

    let output = compressor.compress();

    std::fs::write(&output_path, &output.0)?;

    let output_size = output.0.len();

    // Decoding

    let decoded = compressor.decompress();

    std::fs::write(&decoded_path, &decoded.0)?;

    let percent = (1.0 - output_size as f32 / input_size as f32) * 100.0;

    println!(
        r#"Size (Bytes):

        {input_path} - {}
        {output_path} - {}
        {decoded_path} - {}

        Total compression: {:0.2}%

    "#,
        get_file_size(&input_path)?,
        get_file_size(&output_path)?,
        percent,
        get_file_size(&decoded_path)?
    );

    let new_sha256 = digest(&decoded.0);

    if original_sha256 == new_sha256 {
        println!(
            "Old and new Hashes are same, so decode was {}. Nice!",
            "successful".green().bold()
        );
    } else {
        println!(
            "Old and new Hashes differ, so decode was {}!",
            "unsuccessful".red().bold()
        );
    }

    Ok(())
}
