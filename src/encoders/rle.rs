use std::ops::Div;

use crate::*;

// Upper & lower bounds for number of consecutive characters that induce an RLE replacement
// Lower bound = 4 because aaaa -> (DELIM)4a    4 bytes -> 3 bytes. Saves at least 1 byte
const MIN_REPEAT_COUNT: u8 = 4;

pub struct Rle;

impl Rle {
    pub fn encode(mut input: Vec<u8>) -> Vec<u8> {
        if input.len() == 0 {
            return input;
        }

        let mut output: Vec<u8> = vec![];
        let delim = get_least_used_byte(&input);
        log::info!("Using {delim} ({}) as delim", char::from(delim));

        insert_before_target(&mut input, b'\\', b'\\');
        insert_before_target(&mut input, delim, b'\\');

        let mut count = 1;
        let mut last_byte = input[0];

        let input_iter = input[1..].iter();
        for &current_byte in input_iter {
            // If we encounter a new byte OR we hit the repeat limit
            // potentially append run length encoded vec to the output
            if current_byte != last_byte || count == u8::MAX {
                if count < MIN_REPEAT_COUNT {
                    // If we're under the minimum repeat count, append the raw bytes as is
                    (0..count).into_iter().for_each(|_| output.push(last_byte));
                } else {
                    // Otherwise, it means MIN <= count <= MAX, so we append the encoding
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
        output
    }

    pub fn decode(input: Vec<u8>) -> Vec<u8> {
        if input.len() < 2 {
            return input;
        }

        let (&delim, bytes) = input.split_first().unwrap();
        log::info!("Found delim {delim}");

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

        output
    }
}
