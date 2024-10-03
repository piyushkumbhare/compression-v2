use super::encoder::Tokens;

// Huffman Encoding

pub trait Huff {
    fn encode(&mut self) -> &mut Self;
    
    fn decode(&mut self) -> &mut Self;
}

impl Huff for Tokens {
    fn encode(&mut self) -> &mut Self {
        todo!()
    }

    fn decode(&mut self) -> &mut Self {
        todo!()
    }
}