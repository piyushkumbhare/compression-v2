use std::{
    collections::HashMap,
    fmt::{Debug, Display, Write},
};

use colored::Colorize;

use super::encoder::Tokens;
use super::huff_helper::*;

// Huffman Encoding

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
        let mut freq_map: Vec<EncodeNode> = freq_map
            .iter()
            .enumerate()
            .filter_map(|(byte, &count)| match count > 0 {
                true => Some(EncodeNode {
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
            let combined_node = EncodeNode {
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

        // Serialize the huffman tree

        let mut preorder = vec![];
        pre_order(&root, &mut preorder);

        let mut inorder = vec![];
        in_order(&root, &mut inorder);

        let mut ser_pre = vec![];
        for node in preorder {
            match node {
                Some(b) => {
                    ser_pre.push(1);
                    ser_pre.push(b);
                }
                None => ser_pre.push(0),
            }
        }

        let mut ser_in = vec![];
        for node in inorder {
            match node {
                Some(b) => {
                    ser_in.push(1);
                    ser_in.push(b);
                }
                None => ser_in.push(0),
            }
        }

        println!("Pre: {:?}", ser_pre);
        println!("In: {:?}", ser_in);

        // We prepend the length of the file and tree (as u64s)
        // This tells the decoder where the padding zeroes begin in both the tree and the data.
        let mut header = Vec::new();

        let mut file_len: Vec<u8> = (self.0.len() as u64).to_be_bytes().into();
        header.append(&mut file_len);

        let mut tree_len: Vec<u8> = (ser_pre.len() as u64).to_be_bytes().into();
        header.append(&mut tree_len);

        header.append(&mut ser_pre);
        header.append(&mut ser_in);

        // Encode the actual data via Huffman Coding
        // Use a HashMap to cache paths, and perform the traversal on a miss
        // The path will then be encoded into a byte, with up to
        // 7 extra bits being added as padding (all will be 0's)
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

        // Decode the header which contains the following in order:
        // file length: 8 bytes
        // tree length: 8 bytes
        // preorder: tree_len bytes
        // inorder: tree_len bytes
        // data: rest of the file (we stop reading bits after file_len bits)
        let (file_len, rest) = self.0.split_at(8);
        let file_len = u64::from_be_bytes(file_len.try_into().unwrap());

        let (tree_len, rest) = rest.split_at(8);
        let tree_len = u64::from_be_bytes(tree_len.try_into().unwrap());

        let (preorder, rest) = rest.split_at(tree_len as usize);
        let (inorder, data) = rest.split_at(tree_len as usize);

        println!("File is {file_len} bytes long");
        println!("Tree is {tree_len} bytes long");

        let mut pre_iter = preorder.iter();
        let mut preorder = vec![];
        while let Some(&byte) = pre_iter.next() {
            match byte {
                0 => {
                    preorder.push(None);
                }
                1 => {
                    preorder.push(Some(*pre_iter.next().unwrap()));
                },
                _ => panic!("Found an unexpected byte"),
            }
        }

        let mut in_iter = inorder.iter();
        let mut inorder = vec![];
        while let Some(&byte) = in_iter.next() {
            match byte {
                0 => {
                    inorder.push(None);
                }
                1 => {
                    inorder.push(Some(*in_iter.next().unwrap()));
                },
                _ => panic!("Found an unexpected byte"),
            }
        }

        println!("{:?}", preorder);
        println!("{:?}", inorder);
        
        let (pre_mapped, in_mapped) = map_to_reconstruct(preorder, inorder);
        
        println!("Mapped Preorder: {:?}", pre_mapped);
        println!("Mapped Inorder: {:?}", in_mapped);
        
        let mapped_root = build_tree(&pre_mapped[..], &in_mapped[..]);

        println!("{:#?}", mapped_root);


        println!("Data:");
        data.iter().for_each(|b| print!("{:08b} ", b));
        println!();

        todo!()
    }
}
