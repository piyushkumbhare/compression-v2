use super::encoder::Tokens;
use crate::index_of;

/*
    This MTF Encoder is based off of an Adaptive-MTF algorithm by Brandon Simmons.
    http://brandon.si/code/an-adaptive-move-to-front-algorithm/

    Instead of a traditional MTF, where a known "alphabet" is decided beforehand (i.e. ASCII 0..128),
    this approach dynamically builds the alphabet in order to achieve better compression.

    The reason this adaptive appraoch can perform better is because the ideal "alphabet" for MTF is one where the
    most common characters are near the front of the list, which results in an encoded list
    with long strings of single-digit numbers.

    This comes at the cost of including the alphabet in the file itself so it can be decoded.
    However, the true size cost of this "key" is only the number of unique characters in the
    orginal string, meaning it is usually upper-bounded by 256.



    Code-wise, the problem with this current appraoch is that it encodes a string into another string.
    This is bad because it needs a space/delim between each integer, taking up more space.
    This results in nearly a -100% compression rate.

    A potential solution to this is to map from &str -> (String, Vec<usize>).
    The tuple is needed to pass along the unique dictionary generated through the
    Adaptive version of MTF. However, the Vec<usize> would allow some optimization when passing along to an RLE encoder

    But... this raises the inconvenience that this Tuple return value makes MTF's encode incompatible
    with the compression pipeline macro since it expects &dyn Fn(&str) -> String.
    And I really like the pipeline/macro I made... :(
*/


pub trait Mtf {
    fn encode_mtf(&mut self) -> &mut Self;

    fn decode_mtf(&mut self) -> &mut Self;
}

/*
    This MTF Encoder is based off of an Adaptive-MTF algorithm by Brandon Simmons.
    http://brandon.si/code/an-adaptive-move-to-front-algorithm/

    Instead of a traditional MTF, where a known "alphabet" is decided beforehand,
    this approach dynamically builds the alphabet in order to achieve better compression.
    This comes at the cost of including the alphabet in the file itself so it can be decoded.
    The true "cost" of this "key" is only the number of unique characters in the orginal string,
    meaning it is upper-bounded by 256 usually.
*/
impl Mtf for Tokens {
    fn encode_mtf(&mut self) -> &mut Self {
        if self.0.len() == 0 {
            return self
        }

        // Start with empty alphabet and append to it as we find more
        let mut alphabet: Vec<u8> = vec![];
        let mut data: Vec<u8> = vec![];
        self.0.iter().for_each(|&byte| match index_of(&alphabet, &byte) {
            Some(index) => {
                alphabet.remove(index);
                alphabet.insert(0, byte);
                // Since we're dealing with bytes, we know the index in the alphabet
                // is limited by 255 and can be held in a u8
                data.push(index as u8);
            }
            None => {
                alphabet.insert(0, byte);
                data.push((alphabet.len() - 1) as u8);
            }
        });
        let mut print_str = String::new();
        alphabet.iter().for_each(|&b| {
            let c = match b {
                32..=126 => char::from(b),
                _ => '.',
            };
            print_str.push(c);
        });
        log::info!("Using alphabet (ASCII representation):");
        log::info!("{print_str}");
        // Indicate the end of the alphabet by appending the first byte again
        alphabet.push(*alphabet.first().expect("There should have been an alphabet lol..."));

        // Append the header & data together
        alphabet.append(&mut data);
        self.0 = alphabet;
        self
    }

    fn decode_mtf(&mut self) -> &mut Self {
        let mut alphabet: Vec<u8> = vec![];
        let mut output: Vec<u8> = vec![];
        let mut indices: &[u8] = &[];
        
        // Split the input at the second occurance of the first byte
        for (index, &byte) in self.0.iter().enumerate() {
            if alphabet.len() > 1 && *alphabet.first().unwrap() == byte {
                indices = self.0.get(index + 1..).unwrap();
                break;
            }
            alphabet.push(byte);
        }
        let mut print_str = String::new();
        alphabet.iter().for_each(|&b| {
            let c = match b {
                32..=126 => char::from(b),
                _ => '.',
            };
            print_str.push(c);
        });
        log::info!("Found alphabet (ASCII representation):");
        log::info!("{print_str}");
        let indices: Vec<u8> = indices.into();

        for &index in indices.iter().rev() {
            let head = alphabet.remove(0);
            alphabet.insert(index as usize, head);
            output.push(head);
        }
        self.0 = output.into_iter().rev().collect();
        self
    }

}

