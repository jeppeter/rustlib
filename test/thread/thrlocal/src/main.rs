use std::cell::{Cell, RefCell};
//use std::borrow::BorrowMut;
use backtrace::{Backtrace};

struct CCval {
	pub val :u32,
}

impl Default for CCval {
	fn default() -> Self {
		Self::new(1)
	}
}



impl Drop for CCval {
	fn drop(&mut self) {
		let bk = Backtrace::new();
		println!("drop {}\n{:?}",self.val,bk);
	}
}


impl CCval {
	fn new(val :u32) -> Self {
		Self {
			val :val,
		}
	}

	fn to_string(&self) -> String {
		format!("{}",self.val)
	}
}

thread_local! {
	pub static FOO: Cell<CCval> = {
		let bk = Backtrace::new();
		println!("{:?}", bk);
		Cell::new(CCval::new(3))
	};

	static BAR: RefCell<Vec<f32>> = RefCell::new(vec![1.0, 2.0]);
}

fn main() {
	println!("FOO {}",FOO.take().to_string());
	FOO.set(CCval::new(5));
	println!("FOO again {}",FOO.take().to_string());
	let c = std::thread::spawn(move || {
		println!("THREAD FOO {}",FOO.take().to_string());
	});
	let _ = c.join();
	return;
}