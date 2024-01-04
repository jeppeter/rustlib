use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time;

fn  main()  {
	let foo = Arc::new(vec![3.2,63.2,500.2,31.11]);
	let mut thrs : Vec<JoinHandle<_>> = Vec::new();
	//let mut i :i32;
	//i  = 0;
	for j in 0..10 {
		let v = Arc::clone(&foo);
		let vi = j;
		let thr = thread::spawn(move || {
			for i in 0..v.len() {
				println!("thread[{}].[{}]=[{}]",vi,i,v[i]);
				thread::sleep(time::Duration::from_millis(1));
			}
		});
		thrs.push(thr);
	}

	for i  in  0..foo.len() {
		println!("[{}]=[{}]",i,foo[i]);
		thread::sleep(time::Duration::from_millis(1));
	}

	for i in 0..thrs.len() {
		println!("[{}]=[{:?}]",i,thrs[i]);
	}

	for v in thrs {
		v.join().unwrap();
	}

	return;
}