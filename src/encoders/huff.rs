use colored::Colorize;

use super::encoder::Tokens;

// Huffman Encoding

#[derive(Debug)]
enum Node {
    Leaf(u8),
    Internal,
}

#[derive(Debug)]
struct HuffNode {
    node_type: Node,
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
                        node_type: Node::Leaf(byte as u8),
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
                node_type: Node::Internal,
                frequency: n1.frequency + n2.frequency,
                left: Some(Box::new(n1)),
                right: Some(Box::new(n2)),  
            };

            println!("{} {:?}", "Pushed".green(), combined_node);
            freq_map.push(combined_node);
        }
        
        println!("{:?}", freq_map);
        println!("{}", freq_map[0].frequency);


        todo!()
    }

    fn decode_huff(&mut self) -> &mut Self {
        todo!()
    }
}