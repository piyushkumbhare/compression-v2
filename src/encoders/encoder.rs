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
pub enum Encode {
    Bwt,
    Rle,
    Mtf,
}

pub struct Compress<'a> {
    data: Tokens,
    pipeline: &'a [Encode],
}

impl<'a> Compress<'a> {
    pub fn new(data: Vec<u8>, pipeline: &'a [Encode]) -> Self {
        Self {
            data: Tokens::new(data),
            pipeline,
        }
    }

    pub fn compress(&mut self) -> &Tokens {
        for encoder in self.pipeline.iter() {
            match encoder {
                Encode::Bwt => self.data.encode_bwt(),
                Encode::Rle => self.data.encode_rle(),
                Encode::Mtf => self.data.encode_mtf(),
            };
        }
        return &self.data;
    }

    pub fn decompress(&mut self) -> &Tokens {
        for encoder in self.pipeline.iter().rev() {
            match encoder {
                Encode::Bwt => self.data.decode_bwt(),
                Encode::Rle => self.data.decode_rle(),
                Encode::Mtf => self.data.decode_mtf(),
            };
        }
        return &self.data;
    }
}
