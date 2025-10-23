use rusty_experiments::lib::print_hello_world;

use std::fs::*;
use std::io::{Read,Error};
use std::io::BufReader;

/// fn that takes an iterator, and returns an iterator

fn main() -> Result<(), Error> {
    let input_file = File::open("README.md")?;
    let mut reader = BufReader::new(input_file);
    let mbr : MyBR<_> = MyBR(reader);

    let chunk_size = 5;
    let mut output = File::create("output.txt")?;
    let mut bytes_read = 0;

    let mut buffer = vec![0u8; chunk_size];

    // loop {
    //	let bread = reader.read(&mut buffer)?;
    //	bytes_read += bread;
    //	println!("bytes read is {}",bread);
    //	if bread < chunk_size { break }
    // }
    /*
	let encrypted_blocks : Vec<Vec<u8>> = blocks.iter().map(|b| {
	    // copy from the block
	    let mut buf : Vec<u8>  = b.to_vec();
	    buf.resize(blocksize,0u8); // zero pad if shorter than blocksize
	    key.apply_keystream(&mut buf); // encrypt
	    return buf;
	} ).collect();
    */
    println!("total bytes read is {:?}",bytes_read);
    print_hello_world();
    Ok(())
}

// newtype MyBR = BufRead
struct MyBR<R>(BufReader<R>) where R: Read;

impl<R> Iterator for MyBR<R>
where R: Read
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
	let mut buf : Vec<u8> = Vec::with_capacity(512);
	let bytes_read = self.0.read_exact(&mut buf);
	match bytes_read {
	    Ok(_) => Some(buf),
	    Err(_) => None,
	}
    }
}

// impl <R> Iterator for BufReader<R> where R : Read {
//     type Item = Vec<u8>;

//     fn next(&mut self) -> Option<Self::Item> {
//	todo!()
//     }
// }
