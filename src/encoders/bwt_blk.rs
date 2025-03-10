use rayon::prelude::*;

use super::{bwt::Bwt, encoder::Encoder};
use crate::utils::*;

const BLOCK_SIZE_POWER: usize = 10;

#[derive(Clone, Debug)]
pub struct BwtBlk;

impl Encoder for BwtBlk {
    fn encode(&self, input: Vec<u8>) -> Vec<u8> {
        input
            .into_par_iter()
            .chunks(1 << BLOCK_SIZE_POWER)
            .map(|c| {
                let b = Bwt;
                b.encode(c.into())
            })
            .flatten()
            .collect()
    }

    fn decode(&self, input: Vec<u8>) -> Vec<u8> {
        // Chunk size is +8 bytes because the BWT adds a u64 indexing value at the start
        input
            .into_par_iter()
            .chunks((1 << BLOCK_SIZE_POWER) + 8)
            .map(|c| {
                let b = Bwt;
                b.decode(c.into())
            })
            .flatten()
            .collect()
    }
}
