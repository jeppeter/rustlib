
extern "C" {
	fn abs(i :i32) -> i32;
}

fn main() {
	unsafe {
		println!("abs call [{}]",abs(-5));	
	}
    
}
