use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}


/*
this example from 
https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
*/
pub fn main() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    println!("a: {}, b: {}",test1.as_ref().a(), test1.as_ref().b());
    unsafe {
    	std::mem::swap(&mut test1.as_mut().get_unchecked_mut().a,&mut test2.as_mut().get_unchecked_mut().a);	
    }
    
    println!("test2 a: {}, b: {}",test2.as_ref().a(), test2.as_ref().b());
    println!("test1 a: {}, b: {}",test1.as_ref().a(), test1.as_ref().b());

}