use std::env;

fn main() {
	let mut i=0;
    for arg in env::args() {
    	println!("[{}]=[{}]", i, arg);
    	i += 1;
    }
}
