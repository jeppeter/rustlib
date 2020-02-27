use std::sync::{Arc, Barrier};
use std::thread;
use std::time;
fn  main()  {
	let mut handles = Vec::with_capacity(10);
	let barrier = Arc::new(Barrier::new(10));
	for i in 0..10 {
	    let c = barrier.clone();
	    handles.push(thread::spawn(move|| {
	        // do some work
	        println!("i [{}]", i);
	        thread::sleep(time::Duration::from_millis(100  * (i+1)));
	        //c.wait();
	        println!("after i [{}]", i);
	        c.wait();
	    }));
	}
	// Wait for other threads to finish.
	for handle in handles {
	    handle.join().unwrap();
	}
	println!("wait all over");
}