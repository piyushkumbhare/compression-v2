use std::error::Error;



pub struct Tokens(pub Vec<u8>);

impl Tokens {
    pub fn new(input: Vec<u8>) -> Self {
        Self(input)
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        self.0
    }
}
