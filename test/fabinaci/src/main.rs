use std::env;

fn main() {
	let mut c :i32;
	for arg in env::args() {
		c = arg.parse().unwrap_or(0);
		println!("[{}]=[{}]", arg, fabonaci(c));
	}
}


fn fabonaci(i: i32) -> i32 {
	if  i > 1 {
		return fabonaci(i-1) + i;
	} 
	return i;	
}