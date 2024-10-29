// #![allow(unused)]
mod encoders;
mod tests;
mod utils;

use encoder::{Encoding, Tokens};
use encoders::*;
use simple_logger::SimpleLogger;
use utils::*;

use std::{error::Error, io::Write, process::exit};

use clap::Parser;
use colored::Colorize;
use sha256::digest;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI Args
    let args = Args::parse();

    // If printing to stdout OR if quiet option enabled, then don't enable log printing
    if !args.stdout && !args.quiet {
        SimpleLogger::new().without_timestamps().init()?;
    }

    // Get input data. Hash it for decode verification later.
    let input_data = std::fs::read(&args.input_path)?;
    let input_size = input_data.len();
    let original_sha256 = digest(&input_data);

    // Define the Encoding pipeline
    let pipeline = match args.pipeline {
        Some(e) => {
            use Encoding::*;
            e.iter().map(|s| {
                match s.to_uppercase().as_str() {
                    "BWT" => Bwt,
                    "MTF" => Mtf,
                    "RLE" => Rle,
                    "HUFF" => Huff,
                    _ => panic!("Found an unexpected Encoding value with the --pipeline option. Use --help for more info"),
                }
            }).collect()
        }
        None => {
            use Encoding::*;
            vec![Bwt, Mtf, Rle, Huff]
        }
    };

    let mut compressor = Tokens::new(input_data);

    // Declare output data, which will vary & change based on argument flags
    let output_data;
    let mut output_file;
    enum OutputFile {
        File(String),
        Stdout,
    }

    match args.decompress {
        true => {
            // If decompressing, then verify file ends with .pkz, then trim that to get output path
            if let Some(output_path) = args.input_path.strip_suffix(".pkz") {
                output_file = OutputFile::File(output_path.to_string());
                output_data = compressor.decompress();
            } else {
                log::error!("This program expects a .pkz file when decompressing!");
                panic!("This program expects a .pkz file when decompressing!");
            }
        }
        false => {
            // If compressing, simply append .pkz to input path
            log::info!("Using Encoding Pipeline: {:?}", &pipeline);
            output_file = OutputFile::File(format!("{}.pkz", args.input_path));
            output_data = compressor.compress(&pipeline);
        }
    }

    let output_size = output_data.0.len();

    // If --stdout option was passed, then overwrite the output file
    match args.stdout {
        true => {
            output_file = OutputFile::Stdout;
        }
        false => {}
    }

    match output_file {
        OutputFile::File(ref s) => {
            std::fs::write(s.as_str(), &output_data.0)?;
        }
        OutputFile::Stdout => {
            std::io::stdout().write(&output_data.0)?;
        }
    };

    if args.check && !args.decompress {
        let decoded = match args.decompress {
            true => compressor.compress(&pipeline),
            false => compressor.decompress(),
        };

        let new_sha256 = digest(&decoded.0);

        if original_sha256 == new_sha256 {
            log::info!(
                "Decode: {}. Compressed file decodes back to original.",
                "Success".green().bold()
            );
            exit(0);
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
    }

    if !args.decompress {
        // Print statistics & results
        let output_path = match output_file {
            OutputFile::File(ref s) => s,
            OutputFile::Stdout => "stdout",
        };

        let percent = (1.0 - output_size as f32 / input_size as f32) * 100.0;

        log::info!(
            r#"
    Size (Bytes):

        {} - {input_size} bytes
        {output_path} - {output_size} bytes

    Total compression: {:0.2}%
    
    {}
    "#,
            args.input_path,
            percent,
            format!(
                "The output file is {:0.2}% of its original size!",
                100.0 - percent
            )
            .bold(),
        );
    }
    Ok(())
}
