use std::env;
use std::fs;
use std::io::{self, BufReader, BufRead};

fn main() {
    let input = env::args().nth(1);
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };
    for line in reader.lines() {
    	match line {
    		Ok(l) => {println!("{}",l );},
    		Err(e) => {eprintln!("error [{:?}]",e);}
    	}
    }
}