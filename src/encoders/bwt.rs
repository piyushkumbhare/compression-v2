use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::time::SystemTime;
use std::{collections::HashMap, error::Error};

use super::encoder::Encoder;
use crate::utils::*;
use suffix_array::SuffixArray;

#[derive(PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
enum BwtToken {
    Delim,
    Byte(u8),
}

impl Debug for BwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delim => write!(f, "Delim"),
            Self::Byte(arg0) => f.write_fmt(format_args!("{arg0}")),
        }
    }
}

impl Ord for BwtToken {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (BwtToken::Delim, BwtToken::Delim) => Ordering::Equal,
            (BwtToken::Delim, BwtToken::Byte(_)) => Ordering::Less,
            (BwtToken::Byte(_), BwtToken::Delim) => Ordering::Greater,
            (BwtToken::Byte(b1), BwtToken::Byte(b2)) => b1.cmp(b2),
        }
    }
}

#[derive(Debug)]
pub struct ParseError<'a>(&'a str);

impl<'a> Error for ParseError<'_> {}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error while parsing data. File may be corrupted.\n")?;
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Bwt;

impl Encoder for Bwt {
    fn encode(&self, input: Vec<u8>) -> Vec<u8> {
        let time = SystemTime::now();

        // let mut tokens: Vec<Token> = input.iter().map(|&b| Token::Byte(b)).collect();
        // tokens.push(Token::Delim);

        // let mut suffix_array: Vec<(usize, &[Token])> = tokens
        //     .iter()
        //     .enumerate()
        //     .map(|(i, _)| &tokens[i..])
        //     .enumerate()
        //     .collect();

        // suffix_array.sort_by_key(|(_index, token)| *token);
        // let suffix_array: Vec<usize> = suffix_array
        //     .into_iter()
        //     .map(|(index, _token)| index)
        //     .collect();

        let suffix_array: Vec<u32> = SuffixArray::new(&input).into_parts().1;

        let elapsed = time.elapsed().unwrap();
        log::debug!(
            "Finished creating Suffix Array in {} ms!",
            elapsed.as_millis()
        );

        let mut delim_pos: usize = 0;
        let mut encoded_output: Vec<u8> = vec![];

        for (index, position) in suffix_array.iter().enumerate() {
            if *position > 0 {
                encoded_output.push(input[*position as usize - 1]);
            } else {
                delim_pos = index;
            }
        }

        log::debug!("Encoding: Placing delim at {:#0x} {delim_pos}", delim_pos);

        let delim_pos = delim_pos as u64;
        let mut output: Vec<u8> = delim_pos.to_be_bytes().into();
        output.append(&mut encoded_output);
        output
    }

    // FIXME: This function is hella slow ngl
    fn decode(&self, input: Vec<u8>) -> Vec<u8> {
        // The first 8 bytes are the index of the BWT
        let (header, data) = input.split_at(8);

        // TODO: Fix this cursed ass code
        // Actually maybe we need some cursed ass code in today's society...
        let header: [u8; 8] = *header.first_chunk::<8>().unwrap();
        let delim_pos = usize::from_be_bytes(header);

        log::debug!("Decoding: Placing delim at {:#0x} {delim_pos}", delim_pos);

        let mut unsorted = tools::enumerate_duplicate_bytes(data);

        let mut sorted = unsorted.clone();
        radsort::sort(&mut sorted);

        // Now we convert to Tokens
        let mut unsorted: Vec<(BwtToken, usize)> = unsorted
            .iter()
            .map(|&(b, c)| (BwtToken::Byte(b), c))
            .collect();

        let mut sorted: Vec<(BwtToken, usize)> = sorted
            .iter()
            .map(|&(b, c)| (BwtToken::Byte(b), c))
            .collect();

        // Insert the Delim token at its known positions
        unsorted.insert(delim_pos, (BwtToken::Delim, 0));
        sorted.insert(0, (BwtToken::Delim, 0));

        let mut map: HashMap<(BwtToken, usize), (BwtToken, usize)> = HashMap::new();
        sorted.iter().zip(&unsorted).for_each(|(p1, p2)| {
            map.insert(p1.clone(), p2.clone());
        });

        let mut decoded_tokens: Vec<BwtToken> = Vec::with_capacity(unsorted.len());
        let mut current_byte = (BwtToken::Delim, 0 as usize);

        drop(sorted);
        drop(unsorted);

        // Backtrack through the BWT dictionary to rebuild original string
        while *decoded_tokens.last().unwrap_or(&BwtToken::Byte(0)) != BwtToken::Delim {
            let next_byte = map
                .remove(&current_byte)
                .expect("A byte was read that was really not supposed to be there...");
            decoded_tokens.push(next_byte.0);
            current_byte = next_byte;
        }

        let output: Vec<u8> = decoded_tokens
            .iter()
            .rev()
            .filter_map(|&token| match token {
                BwtToken::Byte(b) => Some(b),
                BwtToken::Delim => None,
            })
            .collect();

        output
    }
}
