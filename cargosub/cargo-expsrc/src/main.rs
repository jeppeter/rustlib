use std::env;

fn main() {
	let mut i :usize = 0;
    for v in env::args() {
    	println!("[{}]=[{}]",i,v);
    	i += 1;
    }
}
