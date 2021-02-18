
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use atomic_refcell::AtomicRefCell;
//use std::cell::RefCell;


fn main() {
	let v = Arc::new(AtomicRefCell::new(vec![1,2,3]));
	let a = v.clone();
	let dlock = Arc::new(Mutex::new(0));
	let reslock = Arc::clone(&dlock);

	let handle = thread::spawn(move || {
		for i in 0..10 {
			{				
				let mut _d = reslock.lock().unwrap();
				println!("[{}]={:?}",i, a.borrow() );
				if a.borrow().len() > 1 { 
					a.borrow_mut().drain(0..1);
				}
			}
			thread::sleep(Duration::from_millis(500));

		}
		
	});

	for i in 20..30 {
		{
			let mut _d = dlock.lock().unwrap();
			println!("[{}]={:?}",i, v.borrow());
			v.borrow_mut().push(i);
		}
		thread::sleep(Duration::from_millis(500));
	}

	handle.join().unwrap();
	println!("after thread v {:?}",v.borrow());
	return;
}