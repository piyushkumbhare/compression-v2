use colored::Colorize;

use super::{bwt::Bwt, huff::Huff, mtf::Mtf, rle::Rle};

#[derive(Clone, Debug)]
pub struct Tokens {
    pipeline: Vec<Encoding>
}

/// Defines the various types of Encoding Algorithms
#[allow(unused)]
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Encoding {
    Bwt = 0,
    Mtf = 1,
    Rle = 2,
    Huff = 3,
}

impl TryFrom<u8> for Encoding {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bwt),
            1 => Ok(Self::Mtf),
            2 => Ok(Self::Rle),
            3 => Ok(Self::Huff),
            _ => Err(()),
        }
    }
}

impl Tokens {
    pub fn new(pipeline: Vec<Encoding>) -> Self {
        Self { pipeline }
    }

    pub fn compress(&mut self, data: Vec<u8>) -> Vec<u8> {
        let mut output = data;
        for encoder in self.pipeline.iter() {
            match encoder {
                Encoding::Bwt => {
                    log::info!("=====[{} - BWT]=====", "ENCODE".green());
                    output = Bwt::encode(output);
                }
                Encoding::Rle => {
                    log::info!("=====[{} - RLE]=====", "ENCODE".green());
                    output = Rle::encode(output);
                }
                Encoding::Mtf => {
                    log::info!("=====[{} - MTF]=====", "ENCODE".green());
                    output = Mtf::encode(output);
                }
                Encoding::Huff => {
                    log::info!("=====[{} - HUFF]====", "ENCODE".green());
                    output = Huff::encode(output);
                }
            };
        }

        let mut encoding_header: Vec<u8> = self.pipeline.iter().map(|&x| x as u8).collect();
        encoding_header.push(b'|');
        encoding_header.append(&mut output);
        encoding_header
    }

    pub fn decompress(&mut self, data: Vec<u8>) -> Vec<u8> {
        let split_index = data
            .iter()
            .position(|&b| b == b'|')
            .expect("Unable to find Encoding Header delimiter '|'");

        
        let mut encode_header = vec![0; split_index];
        encode_header[..].clone_from_slice(&data.split_at(split_index).0);
        let mut output = data;

        output = output.strip_prefix(&encode_header[..]).unwrap().into();
        output = output.strip_prefix(&[b'|']).unwrap().into();

        let pipeline: Vec<Encoding> = encode_header
            .iter()
            .map(|x| {
                Encoding::try_from(*x).expect("Found an unmapped Encoding Enum")
            })
            .collect();

        log::info!("Found Encoding Pipeline: {:?}", &pipeline);

        for encoder in pipeline.iter().rev() {
            match encoder {
                Encoding::Bwt => {
                    log::info!("=====[{} - BWT]=====", "DECODE".cyan());
                    output = Bwt::decode(output);
                }
                Encoding::Rle => {
                    log::info!("=====[{} - RLE]=====", "DECODE".cyan());
                    output = Rle::decode(output);
                }
                Encoding::Mtf => {
                    log::info!("=====[{} - MTF]=====", "DECODE".cyan());
                    output = Mtf::decode(output);
                }
                Encoding::Huff => {
                    log::info!("=====[{} - HUFF]====", "DECODE".cyan());
                    output = Huff::decode(output);
                }
            };
        }
        return output;
    }
}
