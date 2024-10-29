use colored::Colorize;

use super::{bwt::Bwt, huff::Huff, mtf::Mtf, rle::Rle};

#[derive(Clone, Debug)]
pub struct Tokens(pub Vec<u8>);

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
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn compress(&mut self, pipeline: &Vec<Encoding>) -> &Tokens {
        for encoder in pipeline.iter() {
            match encoder {
                Encoding::Bwt => {
                    log::info!("====={} - BWT=====", "ENCODE".green());
                    self.encode_bwt()
                }
                Encoding::Rle => {
                    log::info!("====={} - RLE=====", "ENCODE".green());
                    self.encode_rle()
                }
                Encoding::Mtf => {
                    log::info!("====={} - MTF=====", "ENCODE".green());
                    self.encode_mtf()
                }
                Encoding::Huff => {
                    log::info!("====={} - HUFF====", "ENCODE".green());
                    self.encode_huff()
                }
            };
        }

        let mut encoding_header: Vec<u8> = pipeline.iter().map(|&x| x as u8).collect();
        encoding_header.push(b'|');
        encoding_header.append(&mut self.0);
        self.0 = encoding_header;
        return self;
    }

    pub fn decompress(&mut self) -> &Tokens {
        let split_index = self
            .0
            .iter()
            .position(|&b| b == b'|')
            .expect("Unable to find Encoding Header delimiter '|'");

        
        let mut encode_header = vec![0; split_index];
        encode_header[..].clone_from_slice(self.0.split_at(split_index).0);
        
        self.0 = self.0.strip_prefix(&encode_header[..]).unwrap().into();
        self.0 = self.0.strip_prefix(&[b'|']).unwrap().into();

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
                    log::info!("====={} - BWT=====", "DECODE".cyan());
                    self.decode_bwt()
                }
                Encoding::Rle => {
                    log::info!("====={} - RLE=====", "DECODE".cyan());
                    self.decode_rle()
                }
                Encoding::Mtf => {
                    log::info!("====={} - MTF=====", "DECODE".cyan());
                    self.decode_mtf()
                }
                Encoding::Huff => {
                    log::info!("====={} - HUFF====", "DECODE".cyan());
                    self.decode_huff()
                }
            };
        }
        return self;
    }
}
