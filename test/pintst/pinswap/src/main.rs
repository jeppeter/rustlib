/***************************************
examples：
    https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
***************************************/

use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}


impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}

pub fn main1() {
    // test1 is safe to move before we initialize it
    let mut test1 = Test::new("test1");
    // Notice how we shadow `test1` to prevent it from being accessed again
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
}

pub fn main2() {
    let mut test1 = Test::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    unsafe {
    	std::mem::swap(test1.as_mut().get_unchecked_mut(), test2.as_mut().get_unchecked_mut());	
    }    
    /****************************************
    origin :
    	std::mem::swap(test1.get_mut(), test2.get_mut());	
    	will compiled error when get_mut will call check for T:Unpin
    ****************************************/
    println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
}

fn main() {
   let mut test1 = Test::new("test1");
   let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
   Test::init(test1_pin.as_mut());

   drop(test1_pin);
   println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

   let mut test2 = Test::new("test2");
   std::mem::swap(&mut test1, &mut test2);
   println!("... and now it points nowhere: {:?}", test1.b);
}
