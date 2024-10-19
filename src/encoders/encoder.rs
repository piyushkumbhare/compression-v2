use colored::Colorize;

use super::{bwt::Bwt, huff::Huff, mtf::Mtf, rle::Rle};

#[derive(Clone, Debug)]
pub struct Tokens(pub Vec<u8>);

impl Tokens {
    pub fn new(input: Vec<u8>) -> Self {
        Self(input)
    }
}

/// Defines the various types of Encoding Algorithms
#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum Encoding {
    Bwt,
    Rle,
    Mtf,
    Huff,
}

pub struct Compress<'a> {
    data: Tokens,
    pipeline: &'a [Encoding],
}

impl<'a> Compress<'a> {
    pub fn new(data: Vec<u8>, pipeline: &'a [Encoding]) -> Self {
        Self {
            data: Tokens::new(data),
            pipeline,
        }
    }

    pub fn compress(&mut self) -> &Tokens {
        for encoder in self.pipeline.iter() {
            match encoder {
                Encoding::Bwt => {
                    println!("\n====={} - BWT=====", "ENCODE".green());
                    self.data.encode_bwt()
                },
                Encoding::Rle => {
                    println!("\n====={} - RLE=====", "ENCODE".green());
                    self.data.encode_rle()
                },
                Encoding::Mtf => {
                    println!("\n====={} - MTF=====", "ENCODE".green());
                    self.data.encode_mtf()
                },
                Encoding::Huff => {
                    println!("\n====={} - HUFF====", "ENCODE".green());
                    self.data.encode_huff()
                },
            };
        }
        return &self.data;
    }

    pub fn decompress(&mut self) -> &Tokens {
        for encoder in self.pipeline.iter().rev() {
            match encoder {
                Encoding::Bwt => {
                    println!("\n====={} - BWT=====", "DECODE".cyan());
                    self.data.decode_bwt()
                },
                Encoding::Rle => {
                    println!("\n====={} - RLE=====", "DECODE".cyan());
                    self.data.decode_rle()
                },
                Encoding::Mtf => {
                    println!("\n====={} - MTF=====", "DECODE".cyan());
                    self.data.decode_mtf()
                },
                Encoding::Huff => {
                    println!("\n====={} - HUFF====", "DECODE".cyan());
                    self.data.decode_huff()
                },
            };
        }
        return &self.data;
    }
}
