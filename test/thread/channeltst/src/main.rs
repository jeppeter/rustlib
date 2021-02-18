use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
	let (tx,rx) = mpsc::channel();
	let mut chlds = Vec::new();
	let mut chldexits = Vec::new();

	for j in 1..3 {
		let thread_tx = tx.clone();
		let chldexit = Arc::new(AtomicBool::new(true));
		let c = chldexit.clone();
		let chld = thread::spawn(move || {
			for i in 1..=3 {
				let s = format!("thread[{}][{}]",j,i);
				let sres = thread_tx.send(s);
				if sres.is_err() {
					eprintln!("send error[{:?}]", sres);
					break;
				}
				thread::sleep(Duration::from_millis(100));
			}
			println!("thread[{}]exit",j);
			(*c).store(false,Ordering::Relaxed);
		});
		chlds.push(chld);
		chldexits.push(chldexit);
	}

	loop {

		if chlds.len() == 0 {
			break
		}

		let timeout = Duration::from_millis(100);
		let cs = rx.recv_timeout(timeout);
		if cs.is_ok() {
			println!("received [{}]",cs.unwrap());	
		} else {
			println!("received tiemout");
			let mut removed = true;
			while removed {
				let mut idx;
				removed = false;
				idx = 0;
				while idx < chldexits.len() {
					let e = chldexits[idx].clone();
					let bval = (*e).load(Ordering::Relaxed);
					println!("[{}]alived[{}]", idx,bval);
					if ! bval {
						let cv = chlds.remove(idx);
						cv.join().unwrap();
						chldexits.remove(idx);
						removed = true;
						break;
					}
					idx += 1;
				}
			}
		}
		
	}

	println!("received over");


}
