use colored::Colorize;
use queue::Queue;

use super::encoder::Tokens;

// Huffman Encoding


#[derive(Debug, Clone)]
struct HuffNode {
    byte: Option<u8>,
    frequency: usize,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

pub trait Huff {
    fn encode_huff(&mut self) -> &mut Self;
    
    fn decode_huff(&mut self) -> &mut Self;
}

impl Huff for Tokens {
    fn encode_huff(&mut self) -> &mut Self {
        let freq_map: &mut [usize] = &mut [0; u8::MAX as usize];
        self.0.iter().for_each(|&byte| {
            freq_map[byte as usize] += 1;
        });
        
        let mut freq_map: Vec<HuffNode> = freq_map.iter().enumerate().filter_map(|(byte, &count)| {
            match count > 0 {
                true => {
                    Some(HuffNode {
                        byte: Some(byte as u8),
                        frequency: count,
                        left: None,
                        right: None,
                    })
                },
                false => None,
            }
        }).collect();

        println!("{:?}", freq_map);


        while freq_map.len() > 1 {
            // Sort in reverse order (least frequent at end of vec)
            freq_map.sort_by_key(|h| usize::MAX - h.frequency);
            let n1 = freq_map.pop().expect("Expected to find a HuffNode");
            let n2 = freq_map.pop().expect("Expected to find a HuffNode");

            println!("{} {:?}", "Popped".red(), n1);
            println!("{} {:?}", "Popped".red(), n2);
            let combined_node = HuffNode {
                byte: None,
                frequency: n1.frequency + n2.frequency,
                left: Some(Box::new(n1)),
                right: Some(Box::new(n2)),  
            };

            println!("{} {:?}", "Pushed".green(), combined_node);
            freq_map.push(combined_node);
        }
        
        let root = freq_map.pop().expect("Expected there to be more than 0 HuffNodes");
        let root = Some(Box::new(root));
        let mut header = Vec::new();
        encode_huff_tree(&root, &mut header);

        header.iter().for_each(|b| print!("{:08b} ", b));
        println!();

        let data: Vec<u8> = vec![];

        let mut current_byte: u8 = 0x0;
        for &byte in self.0.iter() {
            
        }

        todo!()
    }

    fn decode_huff(&mut self) -> &mut Self {
        todo!()
    }
}

fn encode_huff_tree(node: &Option<Box<HuffNode>>, arr: &mut Vec<u8>) {
    if let Some(ref n) = node {
        arr.push(n.byte.unwrap_or(0));
        encode_huff_tree(&n.left, arr);
        encode_huff_tree(&n.right, arr);
    }
}
