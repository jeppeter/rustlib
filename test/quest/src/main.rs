use std::io;
use std::io::Read;
use std::fs::File;
use std::env;
fn read_username_from_file(fname :String) -> Result<String, io::Error>
{
	let mut s = String::new();
	File::open(fname)?.read_to_string(&mut s)?;
	Ok(s)
}
fn main() {
	for arg in env::args()	 {
		let c = arg;
		let s = read_username_from_file(c);
		println!("{} {:?}",c,s);
	}
}