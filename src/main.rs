#![allow(unused)]
use std::{error::Error, str::FromStr};

use clap::Parser;

mod encoders;
mod tests;

mod utils;
use utils::*;

use args::*;
use colored::Colorize;
use encoders::encoder::{Compressor, EncoderType};
use errors::CompressError::*;
use simple_logger::SimpleLogger;

fn main() -> anyhow::Result<()> {
    // Parse CLI Args
    let args = Args::parse();

    // If --quiet, turn off all logging
    if !args.quiet {
        if args.verbose {
            // If --verbose, Debug logging
            SimpleLogger::new()
                .with_level(log::LevelFilter::Debug)
                .init();
        } else {
            // else, default to Info logging
            SimpleLogger::new()
                .with_level(log::LevelFilter::Info)
                .without_timestamps()
                .init();
        }
    }

    match args.command {
        Mode::Compress {
            file,
            pipeline,
            check_integerity,
            output,
        } => compress(file, pipeline, check_integerity, output)?,
        Mode::Decompress { file, output } => decompress(file, output)?,
    };

    Ok(())
}

/// Main entry point for a `compress` command
fn compress(
    file: String,
    pipeline: Option<Vec<String>>,
    check_integerity: bool,
    output: Option<String>,
) -> anyhow::Result<()> {
    let output_path = output.unwrap_or_else(|| format!("{}.pkz", &file));
    log::info!("Compressing into {}", output_path.bold());

    let pipeline = match pipeline {
        Some(p) => p
            .iter()
            .map(|f| EncoderType::from_str(&f).unwrap())
            .collect(),
        None => vec![
            EncoderType::Bwt,
            EncoderType::Mtf,
            EncoderType::Rle,
            EncoderType::Huff,
        ],
    };

    let data = std::fs::read(&file)?;

    let mut tokens = Compressor::new(pipeline.clone());

    let mut compressed_contents = match check_integerity {
        true => tokens.compress_and_check(data).unwrap(),
        false => tokens.compress(data),
    };

    // Header of the compressed file will be the u8's representing the order of encoders used
    let mut headers: Vec<u8> = pipeline.iter().map(|e| *e as u8).collect();
    // The b'|' separates the header from the body
    headers.push(b'|');
    headers.append(&mut compressed_contents);

    let final_contents = headers;

    std::fs::write(output_path, final_contents)?;

    Ok(())
}

/// Main entry point for a `decompress` command
fn decompress(file: String, output: Option<String>) -> anyhow::Result<()> {
    let output_path = match output {
        Some(s) => s,
        None => match file.strip_suffix(".pkz") {
            Some(s) => s.to_string(),
            None => format!("{file}.dcmp"),
        },
    };
    log::info!("Decompressing into {}", output_path.bold());

    let data = std::fs::read(&file)?;

    let Some(header_index) = tools::index_of(&data, &b'|') else {
        log::error!("Invlaid format! Aborting...");
        std::process::exit(1);
    };

    let (headers, data) = data.split_at(header_index);

    if data.len() < 2 {
        return Err(ParseError("Header found but no body").into());
    }
    let data = &data[1..];

    // Construct the encoder pipeline from the header
    let mut pipeline: Vec<EncoderType> = Vec::new();

    for &e in headers {
        let Ok(encoder) = EncoderType::try_from(e) else {
            return Err(ParseError("Invalid encoder found in header").into());
        };
        pipeline.push(encoder);
    }

    let mut tokens = Compressor::new(pipeline);

    let decompressed_contents = tokens.decompress(data.into());

    std::fs::write(output_path, decompressed_contents)?;

    log::info!("Done decompressing");
    Ok(())
}
