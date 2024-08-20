use std::pin::Pin;
//use std::ops::Deref;
use pin_project_lite::pin_project;

pin_project! {
    struct NewStruct<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

impl<T, U> NewStruct<T, U> {
	fn new(a :T,b :U) -> Self {
		NewStruct{
			pinned : a,
			unpinned :b,
		}
	}

    fn call_method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
    }
}

/*impl<T,U> Deref for NewStruct<T,U> {
	type Target = NewStruct<T,U>;
	fn deref(&self) -> &Self::Target {
		return self;
	}
}*/

fn main() {
	let mut c = NewStruct::<i32,i64>::new(3,3);
	let d = Pin::new(&mut c);
	d.call_method();
}