use std::error::Error;

use super::encoder::*;
use crate::*;

// Upper & lower bounds for number of consecutive characters that induce an RLE replacement
// Lower bound = 4 because aaaa -> (DELIM)4a    4 bytes -> 3 bytes. Saves at least 1 byte
const MIN_REPEAT_COUNT: u8 = 4;

pub trait Rle {
    fn encode_rle(&mut self) -> &mut Self;

    fn decode_rle(&mut self) -> &mut Self;
}

impl Rle for Tokens {
    fn encode_rle(&mut self) -> &mut Self {
        if self.0.len() == 0 {
            return self;
        }

        let mut output: Vec<u8> = vec![];
        let delim = get_least_used_byte(&self.0);

        insert_before_target(&mut self.0, b'\\', b'\\');
        insert_before_target(&mut self.0, delim, b'\\');

        let mut count = 1;
        let mut last_byte = self.0[0];

        let mut input_iter = self.0[1..].iter();
        for &current_byte in input_iter {
            // If we encounter a new byte OR we hit the repeat limit
            // potentially append run length encoded vec to the output
            if current_byte != last_byte || count == u8::MAX {
                if count < MIN_REPEAT_COUNT {
                    // If we're under the minimum repeat count, append the raw bytes as is
                    output.append(&mut vec![last_byte; count as usize]);
                } else {
                    // Otherwise, it means MIN < count <= MAX, so we append the encoding
                    output.push(delim);
                    output.push(count);
                    output.push(last_byte);
                }
                count = 1;
            } else {
                count += 1;
            }
            last_byte = current_byte;
        }
        // Deal with the last byte
        if MIN_REPEAT_COUNT < count {
            output.push(delim);
            output.push(count);
            output.push(last_byte);
        } else {
            output.append(&mut vec![last_byte; count as usize]);
        }

        output.insert(0, delim);
        self.0 = output;
        self
    }

    fn decode_rle(&mut self) -> &mut Self {
        if self.0.len() < 2 {
            return self;
        }

        let (&delim, bytes) = self.0.split_first().unwrap();

        let mut output: Vec<u8> = vec![];

        let mut bytes = bytes.iter().peekable();
        while let Some(&b) = bytes.next() {
            if b == b'\\' {
                if let Some(&next) = bytes.next() {
                    output.push(next);
                }
            } else if b == delim {
                let count = bytes.next().unwrap();
                let byte_to_repeat = bytes.next().unwrap();
                output.append(&mut vec![*byte_to_repeat; *count as usize]);
            } else {
                output.push(b);
            }
        }

        self
    }
}
