#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
use std::rc::Rc;
use std::cell::{RefCell,RefMut};

const N: usize = 10;


#[allow(deprecated)]
fn main() {

    // Spawn a few threads to increment a shared variable (non-atomically), and
    // let the main thread know once all increments are done.
    //
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let data = Arc::new(Mutex::new(0));
    let mut vhdl = vec![]; 
    let pointer = Rc::new(1);
    let shares = Rc::new(RefCell::new("hello ".to_string()));

    {
        let spointer = pointer.clone();
        println!("spointer {}",*spointer);
    }
    {
        let mut hstr :RefMut<String> = shares.borrow_mut();
        hstr.push_str("world");
    }
    println!("shares [{}]",shares.borrow());

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
            let mut i :u32 = 0;
            while i<3000000 {
                *data += 1;
                i += 1;
            }
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
    {
        let lckdata = data.lock().unwrap();
        println!("main data {}", *lckdata);
    }
}