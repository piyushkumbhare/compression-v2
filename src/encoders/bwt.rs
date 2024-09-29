use core::slice;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::ops::Index;
use std::{collections::HashMap, error::Error};

use radsort::Key;

use crate::*;

use super::encoder::Encoder;

#[derive(PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
enum Token {
    Delim,
    Byte(u8),
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delim => write!(f, "Delim"),
            Self::Byte(arg0) => f.write_fmt(format_args!("{arg0}")),
        }
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Token::Delim, Token::Delim) => Ordering::Equal,
            (Token::Delim, Token::Byte(_)) => Ordering::Less,
            (Token::Byte(_), Token::Delim) => Ordering::Greater,
            (Token::Byte(b1), Token::Byte(b2)) => b1.cmp(b2),
        }
    }
}

#[derive(Debug)]
pub struct ParseError<'a>(&'a str);

impl<'a> Error for ParseError<'_> {}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error while parsing data. File may be corrupted.\n");
        f.write_str(&self.0)
    }
}

pub struct BWT;

impl Encoder for BWT {
    fn encode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut tokens: Vec<Token> = input.iter().map(|&b| Token::Byte(b)).collect();
        tokens.push(Token::Delim);
        
        // TODO: Convert this function to O(n) Suffix Array creation
        let mut suffix_array: Vec<(usize, &[Token])> = tokens
            .iter()
            .enumerate()
            .map(|(i, _)| &tokens[i..])
            .enumerate()
            .collect();

        suffix_array.sort_by_key(|(index, token)| *token);
        let suffix_array: Vec<usize> = suffix_array
            .into_iter()
            .map(|(index, token)| index)
            .collect();

        let mut delim_pos: usize = 0;
        let mut encoded_output: Vec<u8> = vec![];

        for (index, position) in suffix_array.iter().enumerate() {
            if *position > 0 {
                encoded_output.push(input[position - 1]);
            } else {
                delim_pos = index;
            }
        }

        let delim_pos_b36 = format_radix(delim_pos as u32, 36);

        println!("Placing delim at position {delim_pos} = {}", delim_pos_b36);

        let mut final_output = format!("{}|", delim_pos_b36).into_bytes();
        final_output.append(&mut encoded_output);

        Ok(final_output)
    }

    fn decode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        // First start by splitting on the first b'|', which separates the header & the data
        let Some(split_index) = input.iter().position(|&b| b == b'|') else {
            return Err(Box::new(ParseError("Unable to find BWT delimiter '|'")));
        };

        let (header, data) = input.split_at(split_index);
        let header: String = header.iter().map(|b| char::from(*b)).collect();
        let Some(data) = data.get(1..) else {
            return Err(Box::new(ParseError("Unable to split bytes at '|'")));
        };

        let delim_pos: usize = usize::from_str_radix(&header, 36)?;

        // Convert all bytes to Tokens & insert the Delim based on header
        let mut tokens: Vec<Token> = data.iter().map(|&b| Token::Byte(b)).collect();
        tokens.insert(delim_pos, Token::Delim);

        let unsorted = enumerate_duplicates(tokens.clone());

        // To use Radix Sort, we must let radsort operate BEFORE we tokenize (since Token can't implement Key)
        // To work around this, we can insert Token::Delim at position 0 since we know it will end up there post-sort
        let mut sorted: Vec<u8> = data.into();
        radsort::sort(&mut sorted);

        // Now we convert to Tokens
        let mut sorted: Vec<Token> = sorted.iter().map(|&b| Token::Byte(b)).collect();
        sorted.insert(0, Token::Delim);

        // Then enumerate duplicates
        let sorted = enumerate_duplicates(sorted);

        let mut map: HashMap<(Token, usize), (Token, usize)> = HashMap::new();
        sorted.iter().zip(&unsorted).for_each(|(p1, p2)| {
            map.insert(p1.clone(), p2.clone());
        });

        let mut decoded_tokens: Vec<Token> = Vec::with_capacity(unsorted.len());
        let mut current_char = (Token::Delim, 0 as usize);

        drop(sorted);
        drop(unsorted);

        // Backtrack through the BWT dictionary to rebuild original string
        while *decoded_tokens.last().unwrap_or(&Token::Byte(0)) != Token::Delim {
            let Some(next_char) = map.remove(&current_char) else {
                return Err(Box::new(ParseError("A byte was read that was really not supposed to be there...")));
            };
            decoded_tokens.push(next_char.0);
            current_char = next_char;
        }

        let output: Vec<u8> = decoded_tokens
            .iter()
            .rev()
            .filter_map(|&token| match token {
                Token::Byte(b) => Some(b),
                Token::Delim => None,
            })
            .collect();

        Ok(output)
    }
}