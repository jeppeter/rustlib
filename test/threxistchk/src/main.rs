use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

fn main() {
    // Play with this flag
    let fatal_flag = false;
    let do_stop = true;

    let working = Arc::new(AtomicBool::new(true));
    let control = Arc::downgrade(&working);
    let alived = Arc::new(AtomicBool::new(true));
    let alivectrl = Arc::downgrade(&alived);
    
    thread::spawn(move || {
        while (*working).load(Ordering::Relaxed) {
            if fatal_flag {
                panic!("Oh, my God!");
            } else {
                thread::sleep(Duration::from_millis(20));
                println!("I'm alive!");
            }
        }
        println!("child exist");
        (*alived).store(false,Ordering::Relaxed);
    });
    
    thread::sleep(Duration::from_millis(50));
    
    if do_stop {
        // To stop thread
        match control.upgrade() {
            Some(working) => (*working).store(false, Ordering::Relaxed),
            None => println!("Sorry, but thread has died."),
        }
    }

    match alivectrl.upgrade() {
		Some(aliv) => {
			while (*aliv).load(Ordering::Relaxed) {
				println!("child alived");
			    thread::sleep(Duration::from_millis(50));
			}
			println!("child detect exit");
		},
		None => {
			println!("not detect child");
		}
    }
    
    
}