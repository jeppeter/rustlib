
use std::thread;
use std::sync::Arc;
fn main() {
	let v = Arc::new(vec![1,2,3]);
	let a = v.clone();

	let handle = thread::spawn(move || {
		println!("{:?}",a);
	});
	handle.join().unwrap();
	println!("after thread v [{:?}]",v);
	return;
}