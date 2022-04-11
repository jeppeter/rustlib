
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
	let mut uidx :usize;

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
        thread::sleep(Duration::from_secs(i));
        println!("thread {} activated!", i);
        stop.signal();
    });
	}

	uidx=0;
	while uidx < startvec.len() {
		startvec[uidx].signal();
		uidx += 1;
	}

	while stopvec.len() > 0 {
		for i in 0..stopvec.len() {
			if stopvec[i].status(){
				stopvec.remove(i);
				println!("remove [{}]",i);
				break;
			}
		}
	}


	println!("all done!");
}