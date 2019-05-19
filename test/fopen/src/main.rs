use std::fs::File;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
fn main() {
	for arg in env::args() {
		read_file(&arg[..]);
	}
	return;
}

fn read_file(f :&str) {
	let fin = File::open(f).unwrap();
	let  fbuf = BufReader::new(&fin);
	for (n,line) in fbuf.lines().enumerate() {
		let l = line.unwrap();
		println!("[{}]{}",n, l);
	}
}
