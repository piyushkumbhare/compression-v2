use std::ops::Div;

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
        println!("Using {delim} as delim");

        insert_before_target(&mut self.0, b'\\', b'\\');
        insert_before_target(&mut self.0, delim, b'\\');

        let mut count = 1;
        let mut last_byte = self.0[0];

        let input_iter = self.0[1..].iter();
        for &current_byte in input_iter {
            // If we encounter a new byte OR we hit the repeat limit
            // potentially append run length encoded vec to the output
            if current_byte != last_byte || count == u8::MAX {
                if count < MIN_REPEAT_COUNT {
                    // If we're under the minimum repeat count, append the raw bytes as is
                    (0..count).into_iter().for_each(|_| output.push(last_byte));
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
        if MIN_REPEAT_COUNT <= count {
            output.push(delim);
            output.push(count);
            output.push(last_byte);
        } else {
            (0..count).into_iter().for_each(|_| output.push(last_byte));
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
        println!("Found delim {delim}");

        let mut output: Vec<u8> = vec![];


        let mut bytes = bytes.iter();
        while let Some(&b) = bytes.next() {
            if b == b'\\' {
                output.push(*bytes.next().unwrap());
            } else if b == delim {
                let mut count = *bytes.next().unwrap();
                let byte_to_repeat = bytes.next().unwrap();
                if *byte_to_repeat == b'\\' {
                    count = count.div(2);
                }
                (0..count)
                    .into_iter()
                    .for_each(|_| output.push(*byte_to_repeat));
            } else {
                output.push(b);
            }
        }

        

        self.0 = output;
        self
    }
}
