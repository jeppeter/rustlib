
use std::mem::ManuallyDrop;

struct CC {
	val :i32,
}

impl Drop for CC {
	fn drop(&mut self) {
		println!("Drop CC {}", self.val);
	}
}

impl CC {
	fn new(val :i32) -> Self {
		CC {
			val : val,
		}
	}

	fn call_print(&self) {
		println!("call CC {}", self.val );
	}
}


fn main() {
    let c :CC = CC::new(35);
    let mut d :ManuallyDrop<CC> = ManuallyDrop::new(CC::new(99));

    c.call_print();
    d.call_print();
    let f :CC;
    unsafe {
    	f = ManuallyDrop::take(&mut d);
    }
    f.call_print();

    return;
}
