use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
fn  main()  {
	let mut handles = Vec::with_capacity(10);
	let lck = Arc::new(Mutex::new(0));
	for i in 0..10 {
	    let cl =  lck.clone();
	    handles.push(thread::spawn(move|| {
	        // do some work
	        match cl.lock()  {
				Ok(guard)  =>  {
		        println!("i [{}]", i);
		        thread::sleep(time::Duration::from_millis(100 ));
		        //c.wait();
		        println!("after i [{}]", i);
		        drop(guard);
				},
				Err(poisoned)  =>  {eprintln!("error  {:?}", poisoned);;  return;},
	        };
	        //cl.unlock();
	    }));
	}
	// Wait for other threads to finish.
	for handle in handles {
	    handle.join().unwrap();
	}
	println!("wait all over");
}