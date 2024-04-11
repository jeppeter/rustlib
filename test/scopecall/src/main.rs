
/*struct ScopeCall {
    c: Vec<Box<dyn FnMut() -> ()>>,
}

impl Drop for ScopeCall {
    fn drop(&mut self) {
        while self.c.len() > 0 {
        	let mut b :Box<dyn FnMut() -> ()> = self.c.pop().unwrap();
        	b();
        }
    }
}

impl ScopeCall {
	pub fn new() -> Self {
		Self {
			c : Vec::new(),
		}
	}

	pub fn push<F : FnMut() -> () + 'static>(&mut self, f :F) -> usize {
		self.c.push(Box::new(f));
		return self.c.len();
	}
}*/


struct ScopeCall<F: FnMut()> {
    c: Option<F>
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        self.c.take().unwrap()()
    }
}

macro_rules! expr { ($e: expr) => { $e } } // tt hack
macro_rules! defer {
    ($($data: tt)*) => (
        let _scope_call = ScopeCall {
            c: Some(|| -> () { expr!({ $($data)* }) })
        };
    )
}


fn main() {
    let mut x = 42u8;

    defer!(println!("defer 1"));
    defer! {
        println!("defer 2");
        x = 90;
        println!("inside defer {}", x)
    }
    println!("normal execution {}", x);
    /*
    let mut b  = ScopeCall::new();
    b.push(|| -> () {
    	println!("defer 1");
    });
    b.push(move || -> () {
    	println!("defer 2");
    	println!("inside defer {}", x);	
    });

    b.push(move || -> () {
    	println!("defer 1");
    	x = 30;
    	println!("defer 1 {}",x);
    });*/
}