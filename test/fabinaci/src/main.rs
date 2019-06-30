use std::env;
use std::vec::Vec;

static mut CNT :i32 =0;	

fn main() {
	let mut c :i64;
	let mut fabv :i64;
	let mut v :Vec<i64> = Vec::new();
	let mut cv :i32;
	for arg in env::args() {
		c = arg.parse().unwrap_or(0);
		fabv = fabonaci(c,&mut v);
		unsafe {
			cv = CNT;
		}
		println!("[{}]=[{}] CNT[{}]", arg, fabv,cv);
	}
}


fn fabonaci<'a>(i: i64, c :&'a mut Vec<i64>) -> i64 {
	let msize :usize ;
	let cc1 :i64;
	let cc2 :i64;
	msize = i as usize;
	unsafe {
		CNT += 1;
	}
	if  i > 2 {
		if c.len() < (msize - 1) {
			fabonaci(i-1,c);
		}
		if c.len() < (msize - 2) {
			fabonaci(i-2,c);
		}
		if c.len() < msize {
			cc1 = c[msize-3];
			cc2 = c[msize-2];
			c.push(cc1 + cc2);
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