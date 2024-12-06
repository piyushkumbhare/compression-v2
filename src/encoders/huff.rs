use std::collections::HashMap;

use super::huff_helper::*;

// Huffman Encoding

pub struct Huff;

impl Huff {
    pub fn encode(input: Vec<u8>) -> Vec<u8> {
        if input.len() == 0 {
            return input;
        }

        // Create an array of zeroes. A byte's frequency = freq_map[byte]
        let freq_map: &mut [usize] = &mut [0; 256 as usize];
        input.iter().for_each(|&byte| {
            freq_map[byte as usize] += 1;
        });

        // Iterate through and construct HuffNodes for all non-zero frequency bytes
        let mut freq_map: Vec<HuffmanNode> = freq_map
            .iter()
            .enumerate()
            .filter_map(|(byte, &count)| match count > 0 {
                true => Some(HuffmanNode {
                    byte: Node::Leaf(byte as u8),
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

        let mut internal_count = 0;
        while freq_map.len() > 1 {
            freq_map.sort();
            // Reverse so push and pop are zero-cost (least freq at end)
            freq_map.reverse();

            let n1 = freq_map.pop().expect("Expected to find a HuffNode");
            let n2 = freq_map.pop().expect("Expected to find a HuffNode");

            // When creating a parent node, convention is to place
            // the smaller of the 2 children on the left
            let combined_node = HuffmanNode {
                byte: Node::Internal(internal_count),
                frequency: n1.frequency + n2.frequency,
                left: Some(Box::new(n1)),
                right: Some(Box::new(n2)),
            };

            internal_count += 1;

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

        let mut prebytes = vec![];
        for node in preorder {
            match node {
                Node::Leaf(b) => {
                    prebytes.push(1);
                    prebytes.push(b);
                }
                Node::Internal(b) => {
                    prebytes.push(0);
                    prebytes.push(b);
                }
            }
        }

        let mut inbytes = vec![];
        for node in inorder {
            match node {
                Node::Leaf(b) => {
                    inbytes.push(1);
                    inbytes.push(b);
                }
                Node::Internal(b) => {
                    inbytes.push(0);
                    inbytes.push(b);
                }
            }
        }

        log::info!("Encoding: File is {} bytes long", input.len());
        log::info!("Encoding: Tree is {} bytes long", prebytes.len());

        // Defines the bytes for the final output.
        let mut file_contents = Vec::new();

        // The final output file will have the following in this order:
        // 1. Serialized Huffman Tree length (in bytes)
        // 2. Pre-order traversal of Huffman Tree
        // 3. In-order traversal of Huffman Tree
        // 4. Expected length of decoded file (in bytes)
        // 5. Encoded data of file
        let mut tree_len: Vec<u8> = (prebytes.len() as u64).to_be_bytes().into();
        file_contents.append(&mut tree_len);

        file_contents.append(&mut prebytes);
        file_contents.append(&mut inbytes);

        let mut file_len: Vec<u8> = (input.len() as u64).to_be_bytes().into();
        file_contents.append(&mut file_len);

        // Encode the actual data via Huffman Coding
        // Retrieve seen paths from a HashMap, manually perform the traversal on a miss
        // The path will then be encoded into a byte, with up to
        // 7 extra bits being added as padding (all will be 0's)
        let mut paths: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut output_data: Vec<u8> = vec![];
        let mut current_byte: u8 = 0;
        let mut num_bits: u8 = 0;

        for &byte in input.iter() {
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
                if num_bits > 7 {
                    output_data.push(current_byte);
                    current_byte = 0;
                    num_bits = 0;
                }

                current_byte |= turn << (7 - num_bits);
                num_bits += 1;
            }
        }
        if num_bits > 0 {
            output_data.push(current_byte);
        }

        file_contents.append(&mut output_data);
        file_contents
    }

    pub fn decode(input: Vec<u8>) -> Vec<u8> {
        if input.len() == 0 {
            return input;
        }

        // Decode the header which contains the following in order:
        // file length: 8 bytes
        // tree length: 8 bytes
        // preorder: tree_len bytes
        // inorder: tree_len bytes
        // data: rest of the file (we stop reading bits after file_len bits)

        let (tree_len, rest) = input.split_at(8);
        let tree_len = u64::from_be_bytes(tree_len.try_into().unwrap());

        let (preorder, rest) = rest.split_at(tree_len as usize);
        let (inorder, rest) = rest.split_at(tree_len as usize);

        let (file_len, data) = rest.split_at(8);
        let file_len = u64::from_be_bytes(file_len.try_into().unwrap());

        log::info!("Decoding: File is {file_len} bytes long");
        log::info!("Decoding: Tree is {tree_len} bytes long");

        let mut pre_iter = preorder.iter();
        let mut preorder = vec![];
        while let Some(&byte) = pre_iter.next() {
            match byte {
                0 => {
                    preorder.push(Node::Internal(*pre_iter.next().unwrap()));
                }
                1 => {
                    preorder.push(Node::Leaf(*pre_iter.next().unwrap()));
                }
                _ => panic!("Found an unexpected byte"),
            }
        }

        let mut in_iter = inorder.iter();
        let mut inorder = vec![];
        while let Some(&byte) = in_iter.next() {
            match byte {
                0 => {
                    inorder.push(Node::Internal(*in_iter.next().unwrap()));
                }
                1 => {
                    inorder.push(Node::Leaf(*in_iter.next().unwrap()));
                }
                _ => panic!("Found an unexpected byte"),
            }
        }

        let root = build_tree(&preorder[..], &inorder[..]);

        let mut data_iter = data.into_iter();
        let mut current_byte = data_iter.next().unwrap();
        let mut num_bits = 0;

        let mut output_data = vec![];
        while output_data.len() < file_len as usize {
            let mut current_node = root.as_ref().unwrap();
            while matches!(current_node.byte, Node::Internal(_)) {
                if num_bits > 7 {
                    current_byte = data_iter.next().unwrap();
                    num_bits = 0;
                }

                match 0x1 & (current_byte >> (7 - num_bits)) {
                    0 => {
                        current_node = current_node
                            .left
                            .as_ref()
                            .expect("Expected to find a Huffman Node here.");
                    }
                    1 => {
                        current_node = current_node
                            .right
                            .as_ref()
                            .expect("Expected to find a Huffman Node here.");
                    }
                    _ => {
                        panic!("Bit level error when &ing with 0x0");
                    }
                }
                num_bits += 1;
            }
            if let Node::Leaf(b) = current_node.byte {
                output_data.push(b);
            } else {
                panic!("Huffman decoding ended on an internal node somehow.")
            }
        }
        output_data
    }
}
