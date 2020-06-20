use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub struct Reader {
    pub reader: Box<dyn Read>,
}

pub fn read_loop(infile: &str, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader = Reader::new(infile)?;
    loop {
        let buffer = match reader.read() {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        let _ = stats_tx.send(buffer.len());
        if write_tx.send(buffer).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());
    Ok(())
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

    fn read(&mut self) -> Result<Vec<u8>> {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = self.reader.read(&mut buffer)?;

        Ok(Vec::from(&buffer[..num_read]))
    }
}
