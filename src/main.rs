// use rusty_experiments::lib::print_hello_world;

use std::fs::*;
use std::io::{Read,Error};
use std::io::BufReader;

/// fn that takes an iterator, and returns an iterator

fn main() -> Result<(), Error> {
    let input_file = File::open("README.md")?;
    let reader = BufReader::new(input_file);
    let mbr : BufReaderIterator<_> = BufReaderIterator{reader, blocksize: 4096};
    let something: Vec<Vec<u8>> = mbr.into_iter().collect();
    println!("{:?}",something);
    // println!("{:?}",mbr.into_iter().next());
    Ok(())
}

#[derive(Debug)]
pub struct BufReaderIterator<R> where R: Read {
    reader: BufReader<R>,
    blocksize: usize,
}

impl<R> BufReaderIterator<R> where R: Read {
    pub fn new(reader: BufReader<R>, blocksize: usize) -> Self {
	Self { reader, blocksize }
    }
}

impl<R> Iterator for BufReaderIterator<R>
where R: Read
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
	let mut buf : Vec<u8> = Vec::with_capacity(self.blocksize);
	buf.resize(self.blocksize, 0u8);
	let bytes_read = self.reader.read(&mut buf);
	match bytes_read {
	    Ok(v) => if v > 0 {Some(buf)} else {None},
	    Err(_) => None,
	}
    }
}
