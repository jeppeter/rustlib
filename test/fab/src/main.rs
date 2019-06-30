use std::env;


static mut CNT :i32 =0;	

fn main() {
	let mut c :i64;
	let mut fabv :i64;
	let mut cnt :i32;
	for arg in env::args() {
		c = arg.parse().unwrap_or(0);
		fabv = fabonaci(c);
		unsafe {
			cnt = CNT;
		}
		println!("[{}]=[{}] CNT[{}]", arg, fabv,cnt);
	}
}


fn fabonaci(i: i64) -> i64 {
	unsafe {
		CNT += 1;
	}
	
	if  i > 2 {
		return fabonaci(i-1) + fabonaci(i-2);
	}  else if i == 2 {
		return 2;
	}	
	return 1;	
}