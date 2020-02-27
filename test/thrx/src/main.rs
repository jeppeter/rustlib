use std::sync::{Arc, Barrier};
use std::thread;
fn  main()  {
	let mut handles = Vec::with_capacity(10);
	let barrier = Arc::new(Barrier::new(10));
	for i in 0..10 {
	    let c = barrier.clone();
	    handles.push(thread::spawn(move|| {
	        // do some work
	        println!("i [{}]", i);
	        c.wait();
	        println!("after i [{}]", i);
	    }));
	}
	// Wait for other threads to finish.
	for handle in handles {
	    handle.join().unwrap();
	}
	println!("wait all over");
}