// use rusty_experiments::lib::print_hello_world;

use std::fs::*;
use std::io::{Read,Error};
use std::io::BufReader;

/// fn that takes an iterator, and returns an iterator

fn main() -> Result<(), Error> {
    let input_file = File::open("README.md")?;
    let reader = BufReader::new(input_file);
    let mbr : MyBR<_> = MyBR(reader);
    let something: Vec<Vec<u8>> = mbr.into_iter().collect();
    println!("{:?}",something);
    // println!("{:?}",mbr.into_iter().next());
    Ok(())
}

#[derive(Debug)]
struct MyBR<R>(BufReader<R>) where R: Read;

impl<R> Iterator for MyBR<R>
where R: Read
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
	let mut buf : Vec<u8> = Vec::with_capacity(8);
	buf.resize(8, 0u8);
	let bytes_read = self.0.read(&mut buf);
	match bytes_read {
	    Ok(v) => if v > 0 {Some(buf)} else {None},
	    Err(_) => None,
	}
    }
}

// impl MyBR<R> {
//     fn new
// }
// impl <R> Iterator for BufReader<R> where R : Read {
//     type Item = Vec<u8>;

//     fn next(&mut self) -> Option<Self::Item> {
//	todo!()
//     }
// }
