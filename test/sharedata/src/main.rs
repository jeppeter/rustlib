#![allow(unused)]
 #[allow(deprecated)]
fn main() {
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

const N: usize = 10;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
let data = Arc::new(Mutex::new(0));
let mut vhdl = vec![]; 

//let (tx, rx) = channel();
for _ in 0..N {
    let data = Arc::clone(&data);
    let hdl = thread::spawn(move || {
        // The shared state can only be accessed once the lock is held.
        // Our non-atomic increment is safe because we're the only thread
        // which can access the shared state when the lock is held.
        //
        // We unwrap() the return value to assert that we are not expecting
        // threads to ever fail while holding the lock.
        let mut data = data.lock().unwrap();
        *data += 1;
        println!("data {}", *data);
        thread::sleep_ms(500);
        /*if *data == N {
            tx.send(()).unwrap();
        }*/
        // the lock is unlocked here when `data` goes out of scope.
    });
    vhdl.push(hdl);
}

for v in vhdl {
	let _ = v.join();
}
}