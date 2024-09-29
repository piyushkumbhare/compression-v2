use std::error::Error;

use super::encoder::Encoder;
use crate::*;

// Upper & lower bounds for number of consecutive characters that induce an RLE replacement
// Lower bound = 4 because aaaa -> (DELIM)4a    4 bytes -> 3 bytes. Saves at least 1 byte
const MIN_REPEAT_COUNT: u8 = 4;

pub struct RLE;

impl Encoder for RLE {
    fn encode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        if input.len() == 0 {
            return Ok(input);
        }

        let mut output: Vec<u8> = vec![];
        let delim = get_least_used_byte(&input);

        let mut count = 1;
        let mut last_byte = input[0];

        let mut input_iter = input[1..].iter();
        loop {
            match input_iter.next() {
                Some(&b) => {
                    // If we encounter a new byte OR we hit the repeat limit
                    // potentially append run length encoded vec to the output
                    if b != last_byte || count == u8::MAX {
                        if count < MIN_REPEAT_COUNT {
                            // If we're under the minimum repeat count, append the raw bytes as is
                            output.append(&mut vec![last_byte; count as usize]);
                        } else {
                            // Otherwise, it means MIN < count <= MAX, so we append the encoding
                            println!("Pushing bytes {delim} {count} {last_byte}");
                            output.push(delim);
                            output.push(count);
                            output.push(last_byte);
                        }
                        count = 1;
                    } else {
                        count += 1;
                    }
                    last_byte = b;
                }
                None => {
                    if MIN_REPEAT_COUNT < count {
                        println!("Pushing bytes {delim} {count} {last_byte}");
                        output.push(delim);
                        output.push(count);
                        output.push(last_byte);
                    } else {
                        output.append(&mut vec![last_byte; count as usize]);
                    }
                    break;
                }
            }
        }

        Ok(output)
    }

    fn decode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}
