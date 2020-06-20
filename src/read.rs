use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub struct Reader {
    pub reader: Box<dyn Read>,
}

impl Reader {
    pub fn new(infile: &str) -> Result<Self> {
        let reader: Box<dyn Read> = if !infile.is_empty() {
            Box::new(BufReader::new(File::open(infile)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };

        Ok(Self { reader })
    }
    pub fn read(&mut self) -> Result<Vec<u8>> {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = self.reader.read(&mut buffer)?;

        Ok(Vec::from(&buffer[..num_read]))
    }
}
