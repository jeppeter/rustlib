use std::error::Error;
use std::fs::File;
//use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
fn main() {
	let mut i :i32=0;
	for arg in env::args() {
		if i > 0 {
			read_file(&arg[..]);	
		}
		i += 1;		
	}
	return;
}

fn read_file(f :&str) {
	let fin =  match File::open(f) {
		Err(why) => panic!("couldn't open {}: {}", f,
                                                   why.description()),
		Ok(fin) => fin,
	};
	let  fbuf = BufReader::new(&fin);
	for (n,line) in fbuf.lines().enumerate() {
		let l = line.unwrap();
		println!("[{}]{}",n, l);
	}
}
