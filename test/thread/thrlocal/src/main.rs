use std::cell::{Cell, RefCell};
//use std::borrow::BorrowMut;
use backtrace::{Backtrace};

thread_local! {
	pub static FOO: Cell<u32> = {
		let bk = Backtrace::new();
		println!("{:?}", bk);
		Cell::new(1)
	};

	static BAR: RefCell<Vec<f32>> = RefCell::new(vec![1.0, 2.0]);
}

fn main() {
	println!("FOO {}",FOO.get());
	FOO.set(5);
	println!("FOO again {}",FOO.get());
	let c = std::thread::spawn(move || {
		println!("THREAD FOO {}",FOO.get());
	});
	let _ = c.join();
	return;
}