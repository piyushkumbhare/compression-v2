use std::error::Error;



pub trait Encoder {
    fn encode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>;

    fn decode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>;
}

