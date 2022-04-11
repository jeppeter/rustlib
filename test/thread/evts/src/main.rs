
use synchronoise::event::{SignalEvent, SignalKind};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
	//let start_signal = Arc::new(SignalEvent::new(false, SignalKind::Manual));
	//let stop_signal = Arc::new(SignalEvent::new(false, SignalKind::Auto));
	let thread_count = 5;
	let mut startvec :Vec<Arc<SignalEvent>> = Vec::new();
	let mut stopvec :Vec<Arc<SignalEvent>> = Vec::new();
	let mut removed :i32;

	for i in 0..thread_count {
		let curstart = Arc::new(SignalEvent::new(false,SignalKind::Manual));
		let curstop = Arc::new(SignalEvent::new(false,SignalKind::Auto));
		let start = curstart.clone();
		let stop = curstop.clone();
		startvec.push(curstart);
		stopvec.push(curstop);
		thread::spawn(move || {
        //as a Manual-reset signal, all the threads will start at the same time
        start.wait();
        thread::sleep(Duration::from_millis(i * 100));
        println!("thread {} activated!", i);
        stop.signal();
    });
	}

	
	while startvec.len() > 0 {
		startvec[0].signal();
		startvec.remove(0);
	}


	while stopvec.len() > 0 {
		removed = 0;
		for i in 0..stopvec.len() {
			if stopvec[i].status(){
				stopvec.remove(i);
				println!("remove [{}]",i);
				removed = 1;
				break;
			}
		}
		if removed == 0 {
			println!("sleep");
			thread::sleep(Duration::from_millis(50));
		}
	}


	println!("all done!");
}