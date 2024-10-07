use std::{
    collections::HashMap,
    fmt::{Debug, Display, Write},
};

use colored::Colorize;

use super::encoder::Tokens;

// Huffman Encoding

// A node in the Huffman Tree
#[derive(Debug, Clone, Eq, PartialEq)]
struct HuffNode {
    frequency: usize,
    byte: Option<u8>,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

/// Implementing PartialOrd manually is required since
/// the derive macro lexicographically orders the fields of the struct,
/// which is NOT what we want
impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Instead, we define our own Ordering for a HuffNode.
/// 2 Nodes are compared in the following order of precedence
///
/// 1. Their frequencies are compared
/// 2. Internal/Leaf status. Internal < Leaf
/// 3. Byte value associated with the Leaf (in the case of Leaf vs Leaf)
impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.frequency != other.frequency {
            self.frequency.cmp(&other.frequency)
        } else {
            use std::cmp::Ordering;
            match (self.byte, other.byte) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(b1), Some(b2)) => b1.cmp(&b2),
            }
        }
    }
}

pub trait Huff {
    fn encode_huff(&mut self) -> &mut Self;

    fn decode_huff(&mut self) -> &mut Self;
}

impl Huff for Tokens {
    fn encode_huff(&mut self) -> &mut Self {
        if self.0.len() == 0 {
            return self;
        }

        // Create an array of zeroes. A byte's frequency = freq_map[byte]
        let freq_map: &mut [usize] = &mut [0; 256 as usize];
        self.0.iter().for_each(|&byte| {
            freq_map[byte as usize] += 1;
        });

        // Iterate through and construct HuffNodes for all non-zero frequency bytes
        let mut freq_map: Vec<HuffNode> = freq_map
            .iter()
            .enumerate()
            .filter_map(|(byte, &count)| match count > 0 {
                true => Some(HuffNode {
                    byte: Some(byte as u8),
                    frequency: count,
                    left: None,
                    right: None,
                }),
                false => None,
            })
            .collect();

        // Huffman Tree creation:
        // Pop smallest 2 frequencies
        // Combine into 1 node
        // Reinsert node
        // Repeat until one node is left
        while freq_map.len() > 1 {
            freq_map.sort();
            // Reverse so push and pop are zero-cost (least freq at end)
            freq_map.reverse();

            let n1 = freq_map.pop().expect("Expected to find a HuffNode");
            let n2 = freq_map.pop().expect("Expected to find a HuffNode");

            // When creating a parent node, convention is to place
            // the smaller of the 2 children on the left
            let combined_node = HuffNode {
                byte: None,
                frequency: n1.frequency + n2.frequency,
                left: Some(Box::new(n1)),
                right: Some(Box::new(n2)),
            };

            // TODO: make this insertion to avoid resorting of array
            freq_map.push(combined_node);
        }

        let root = freq_map
            .pop()
            .expect("Expected there to be more than 0 HuffNodes");
        let root = Some(Box::new(root));
        let mut serialized_tree = Vec::new();
        // Serialize the huffman tree
        serialize_huff_tree(&root, &mut serialized_tree);

        // We prepend the length of the file (as u64)
        // This tells the decoder where the padding zeroes begin.
        let mut header: Vec<u8> = (self.0.len() as u64).to_be_bytes().into();
        header.append(&mut serialized_tree);

        let mut paths: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut output_data: Vec<u8> = vec![];
        let mut current_byte: u8 = 0;
        let mut byte_length: u8 = 0;

        for &byte in self.0.iter() {
            let path = match paths.get(&byte) {
                Some(v) => v,
                None => {
                    let mut path = vec![];
                    encode_byte(&root, byte, &mut path);
                    path.reverse();
                    paths.insert(byte, path);
                    paths.get(&byte).unwrap()
                }
            };

            for turn in path {
                if byte_length > 7 {
                    output_data.push(current_byte);
                    current_byte = 0;
                    byte_length = 0;
                }

                current_byte |= turn << (7 - byte_length);
                byte_length += 1;
            }
        }
        if byte_length > 0 {
            output_data.push(current_byte);
        }

        header.append(&mut output_data);
        self.0 = header;
        self
    }

    fn decode_huff(&mut self) -> &mut Self {
        if self.0.len() == 0 {
            return self;
        }
        let (len, data) = self.0.split_at(8);
        let len = u64::from_be_bytes(len[0..8].try_into().unwrap());
        todo!()
    }
}

/// Serializes the Huffman tree by returning the contents in pre-order traversal.
///
/// Internal nodes are represented by a `0`, while leaf nodes are prefixed by `1`.
fn serialize_huff_tree(node: &Option<Box<HuffNode>>, arr: &mut Vec<u8>) {
    if let Some(ref n) = node {
        match n.byte {
            Some(b) => {
                arr.push(1);
                arr.push(b);
            }
            None => arr.push(0),
        }
        serialize_huff_tree(&n.left, arr);
        serialize_huff_tree(&n.right, arr);
    }
}

/// Recursive backtracking function to trace and return the path to a given byte
/// in the Huffman tree
fn encode_byte(node: &Option<Box<HuffNode>>, byte: u8, path: &mut Vec<u8>) -> bool {
    if let Some(ref n) = node {
        match n.byte {
            Some(b) => b == byte,
            None => {
                let left = encode_byte(&n.left, byte, path);
                let right = encode_byte(&n.right, byte, path);
                if left {
                    path.push(0);
                } else if right {
                    path.push(1);
                }
                return left || right;
            }
        }
    } else {
        false
    }
}
