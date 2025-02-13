use std::fmt::Debug;

use colored::Colorize;
use strum::EnumString;

use super::{bwt::Bwt, huff::Huff, mtf::Mtf, rle::Rle};

pub struct Compressor {
    pipeline: Vec<Box<dyn Encoder>>,
}

pub trait Encoder: Debug {
    fn encode(&self, input: Vec<u8>) -> Vec<u8>;

    fn decode(&self, input: Vec<u8>) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString)]
#[repr(u8)]
pub enum EncoderType {
    Bwt = 0,
    Mtf = 1,
    Rle = 2,
    Huff = 3,
}

impl TryFrom<u8> for EncoderType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Bwt),
            1 => Ok(Self::Mtf),
            2 => Ok(Self::Rle),
            3 => Ok(Self::Huff),
            _ => Err(()),
        }
    }
}

impl Compressor {
    pub fn new(pipeline: Vec<EncoderType>) -> Self {
        let mut encoders: Vec<Box<dyn Encoder>> = Vec::new();

        for encoder_type in pipeline {
            match encoder_type {
                EncoderType::Bwt => encoders.push(Box::new(Bwt)),
                EncoderType::Mtf => encoders.push(Box::new(Mtf)),
                EncoderType::Rle => encoders.push(Box::new(Rle)),
                EncoderType::Huff => encoders.push(Box::new(Huff)),
            }
        }

        Self { pipeline: encoders }
    }

    pub fn compress(&mut self, data: Vec<u8>) -> Vec<u8> {
        let mut output = data;
        for encoder in self.pipeline.iter() {
            log::info!("{}: {:?} started", "Encoding".green(), encoder);
            output = encoder.encode(output);
            log::info!("{}: {:?} finished", "Encoding".green(), encoder);
        }
        output
    }

    pub fn decompress(&mut self, data: Vec<u8>) -> Vec<u8> {
        let mut output = data;
        for encoder in self.pipeline.iter().rev() {
            log::info!("{}: {:?} started", "Decoding".blue(), encoder);
            output = encoder.decode(output);
            log::info!("{}: {:?} finished", "Decoding".blue(), encoder);
        }
        output
    }

    pub fn try_compress(&mut self, data: Vec<u8>) -> Option<Vec<u8>> {
        let original_hash = sha256::digest(&data);

        let compressed = self.compress(data);
        let output = compressed.clone();

        let decompressed = self.decompress(compressed);
        let new_hash = sha256::digest(&decompressed);

        if original_hash == new_hash {
            Some(output)
        } else {
            None
        }
    }
}
