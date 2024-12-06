use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::time::SystemTime;
use std::{collections::HashMap, error::Error};

use suffix_array::SuffixArray;

use crate::utils::*;

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

pub struct Bwt;

impl Bwt {
    pub fn encode(input: Vec<u8>) -> Vec<u8> {
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
        log::info!(
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

        let delim_pos_b36 = format_radix(delim_pos as u32, 36);

        log::info!(
            "Encoding: Placing delim at position {delim_pos} (base 36) = {} (decimal)",
            delim_pos_b36
        );

        let mut output = format!("{}|", delim_pos_b36).into_bytes();
        output.append(&mut encoded_output);
        output
    }

    pub fn decode(input: Vec<u8>) -> Vec<u8> {
        // First start by splitting on the first b'|', which separates the header & the data
        let split_index = input
            .iter()
            .position(|&b| b == b'|')
            .expect("Unable to find BWT delimiter '|'");

        let (header, data) = input.split_at(split_index);
        let header: String = header.iter().map(|b| char::from(*b)).collect();
        let data = data.get(1..).expect("Unable to split bytes at '|'");

        let delim_pos = usize::from_str_radix(&header, 36)
            .expect(format!("Unable to parse `{header}` into a b36 number").as_str());

        log::info!("Decoding: Placing delim at {delim_pos}");
        // Convert all bytes to Tokens & insert the Delim based on header
        let mut tokens: Vec<BwtToken> = data.iter().map(|&b| BwtToken::Byte(b)).collect();
        tokens.insert(delim_pos, BwtToken::Delim);

        let unsorted = enumerate_duplicates(tokens.clone());

        // To use Radix Sort, we must let radsort operate BEFORE we tokenize (since Token can't implement Key)
        // To work around this, we can insert Token::Delim at position 0 since we know it will end up there post-sort
        let mut sorted: Vec<u8> = data.into();
        radsort::sort(&mut sorted);

        // Now we convert to Tokens
        let mut sorted: Vec<BwtToken> = sorted.iter().map(|&b| BwtToken::Byte(b)).collect();
        sorted.insert(0, BwtToken::Delim);

        // Then enumerate duplicates
        let sorted = enumerate_duplicates(sorted);

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
