#![allow(unused)]
mod encoders;
mod tests;
mod utils;

use encoder::{Compress, Encoding};
use encoders::*;
use simple_logger::SimpleLogger;
use utils::*;

use std::{
    error::Error,
    fs::File,
    io::Write,
    os::fd::{AsRawFd, FromRawFd},
};

use clap::Parser;
use colored::Colorize;
use sha256::digest;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if !args.stdout && !args.quiet {
        SimpleLogger::new().without_timestamps().init()?;
    }

    enum OutputFile {
        File(String),
        Stdout,
    }

    let output_file = match args.stdout {
        false => OutputFile::File(format!("{}.pkz", &args.input_path)),
        true => OutputFile::Stdout,
    };

    let input = std::fs::read(&args.input_path)?;

    // Grab some metadata before encoding the input. Used for stats later.
    let original_sha256 = digest(&input);
    let input_size = get_file_size(&args.input_path)?;

    // Define the compression stages to use
    let mut compressor = Compress::new(input, {
        use Encoding::*;
        &[Huff]
    });

    // Perform the compression
    let output = match args.decompress {
        true => compressor.decompress(),
        false => compressor.compress(),
    };
    let output_size = output.0.len();

    // Write the data to the desired output
    match output_file {
        OutputFile::File(ref s) => std::fs::write(s, &output.0),
        OutputFile::Stdout => std::io::stdout().write_all(&output.0),
    }?;

    // Perform the reverse to ensure that you get back to the original input data
    let decoded = match args.decompress {
        true => compressor.compress(),
        false => compressor.decompress(),
    };

    // Print statistics & results
    let percent = (1.0 - output_size as f32 / input_size as f32) * 100.0;

    let output_file = match output_file {
        OutputFile::File(ref s) => s.as_str(),
        OutputFile::Stdout => "dummy",
    };

    let decoded_size = decoded.0.len();

    log::info!(
        r#"
    Size (Bytes):

        {} - {input_size}
        {output_file} - {output_size}
        Decoded size - {decoded_size}

    Total compression: {:0.2}%
    
    {}

    "#,
        args.input_path,
        percent,
        format!(
            "The compressed file is {:0.2}% of its original size!",
            100.0 - percent
        )
        .bold(),
    );

    let new_sha256 = digest(&decoded.0);

    if original_sha256 == new_sha256 {
        log::info!(
            "Decode: {}. Compressed file decodes back to original.",
            "Success".green().bold()
        )
    } else {
        log::error!(
            "Decode: {}. Compressed file does not decode back to original.",
            "Failed".red().bold()
        );
        panic!(
            "Decode: {}. Compressed file does not decode back to original.",
            "Failed".red().bold()
        )
    }
    Ok(())
}
