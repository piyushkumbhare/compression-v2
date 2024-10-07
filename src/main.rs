// #![allow(unused)]
mod encoders;
mod utils;

use encoder::{Compress, Encoding};
use encoders::*;
use utils::*;

use std::error::Error;

use clap::Parser;
use colored::Colorize;
use sha256::digest;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input_path = &args.file;
    let output_path = format!("{}.pkz", &args.file);
    let decoded_path = format!("{}.decoded", &args.file);

    let buf = std::fs::read(input_path)?;

    // Grab some metadata before encoding the input. Used for stats later.
    let original_sha256 = digest(&buf);
    let input_size = get_file_size(&input_path)?;

    // Define the compression stages to use
    let mut compressor = Compress::new(
        buf,
        &[Encoding::Bwt, Encoding::Mtf, Encoding::Rle, Encoding::Huff],
    );

    // Encode & write to `.pkz` file
    let output = compressor.compress();
    std::fs::write(&output_path, &output.0)?;
    let output_size = get_file_size(&output_path)?;

    // Decode & write to `.decoded` file
    let decoded = compressor.decompress();
    std::fs::write(&decoded_path, &decoded.0)?;
    let decoded_size = get_file_size(&decoded_path)?;

    // Print statistics & results
    let percent = (1.0 - output_size as f32 / input_size as f32) * 100.0;

    println!(
        r#"Size (Bytes):

    {input_path} - {input_size}
    {output_path} - {output_size}
    {decoded_path} - {decoded_size}

    Total compression: {:0.2}%

    "#,
        percent,
    );

    let new_sha256 = digest(&decoded.0);

    print!("Decode: ");
    if original_sha256 == new_sha256 {
        println!(
            "{}. Compressed file decodes back to original.",
            "Success".green().bold()
        )
    } else {
        println!(
            "{}. Compressed file does not decode back to original.",
            "Failed".red().bold()
        )
    }

    Ok(())
}
