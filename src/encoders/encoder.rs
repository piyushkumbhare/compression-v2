use super::{bwt::Bwt, mtf::Mtf, rle::Rle};



#[derive(Clone, Debug)]
pub struct Tokens(pub Vec<u8>);

impl Tokens {
    pub fn new(input: Vec<u8>) -> Self {
        Self(input)
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum Encoder {
    Bwt,
    Rle,
    Mtf,
}

pub struct Compress<'a> {
    data: Tokens,
    pipeline: &'a [Encoder],
}

impl<'a> Compress<'a> {
    pub fn new(data: Vec<u8>, pipeline: &'a [Encoder]) -> Self {
        Self {
            data: Tokens::new(data),
            pipeline,
        }
    }

    pub fn compress(&mut self) -> &Tokens {
        for encoder in self.pipeline.iter() {
            match encoder {
                Encoder::Bwt => self.data.encode_bwt(),
                Encoder::Rle => self.data.encode_rle(),
                Encoder::Mtf => self.data.encode_mtf(),
            };
        }
        return &self.data;
    }

    pub fn decompress(&mut self) -> &Tokens {
        for encoder in self.pipeline.iter().rev() {
            match encoder {
                Encoder::Bwt => self.data.decode_bwt(),
                Encoder::Rle => self.data.decode_rle(),
                Encoder::Mtf => self.data.decode_mtf(),
            };
        }
        return &self.data;
    }
}

