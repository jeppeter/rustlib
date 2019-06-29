use std::env;
use std::vec::Vec;

fn main() {
	let mut c :i32;
	let mut v :Vec<i32> = Vec::new();
	let mut cv =  &v;
	for arg in env::args() {
		c = arg.parse().unwrap_or(0);
		println!("[{}]=[{}]", arg, fabonaci(c,cv));
	}
}


fn fabonaci(i: i32, c :&mut Vec<i32>) -> i32 {
	let msize :usize ;
	msize = i as usize;
	if  i > 2 {
		if c.len() < (msize - 1) {
			fabonaci(i-1,c);
		}
		if c.len() < (msize - 2) {
			fabonaci(i-2,c);
		}
		if c.len() < msize {
			c.push(c[msize-3] + c[msize-2]);
		}
		return c[msize-1];
	}  else if i == 2 {
		if c.len() < 1 {
			c.push(1);
		}
		if c.len() < 2 {
			c.push(2);
		}		
		return c[1];
	}
	if c.len() < 1 {
		c.push(1);	
	}
	
	return 1;	
}