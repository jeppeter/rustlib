
use std::cell::UnsafeCell;
use std::sync::Arc;

#[allow(dead_code)]
pub (crate) struct EvtSyncUnsafeCell<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for EvtSyncUnsafeCell<T> {}

impl<T> EvtSyncUnsafeCell<T> {
    /// Constructs a new instance of `EvtSyncUnsafeCell` which will wrap the specified value.
    #[inline]
    pub (crate)  const fn new(value: T) -> Self {
        Self { inner: UnsafeCell::new(value), }
    }

}

#[allow(dead_code)]
impl<T> EvtSyncUnsafeCell<T> {
    /// Gets a mutable pointer to the wrapped value.
    ///
    /// This can be cast to a pointer of any kind.
    /// Ensure that the access is unique (no active references, mutable or not)
    /// when casting to `&mut T`, and ensure that there are no mutations
    /// or mutable aliases going on when casting to `&T`
    #[inline]
    pub (crate) const fn get(&self) -> *mut T {
        self.inner.get()
    }
}

struct ThreadDataInner {
	pub val : i32,
	pub vals :String,
}

impl Drop for ThreadDataInner {
	fn drop(&mut self) {
		self.close();
	}
}

impl ThreadDataInner {

	fn close(&mut self) {
		println!("ThreadDataInner close {} {}",self.val,self.vals);
	}
	fn new(val :i32, vs :&str) -> Self {
		Self {
			val : val,
			vals : format!("{}",vs),
		}
	}
}

#[allow(dead_code)]
#[derive(Clone)]
struct ThreadData {
	pub inner :Arc<EvtSyncUnsafeCell<ThreadDataInner>>,
}

impl Drop for ThreadData {
	fn drop(&mut self) {
		self.close();
	}
}

impl ThreadData {
	fn close(&mut self) {
		println!("ThreadData close ref {}",Arc::strong_count(&self.inner));
	}

	fn new(val :i32, vs :&str) -> Self {
		let retv :Self = Self {
			inner : Arc::new(EvtSyncUnsafeCell::new(ThreadDataInner::new(val,vs))),
		};
		retv
	}
}

fn main() {
    let cv :ThreadData = ThreadData::new(32,"hello");
    {
    	let _bv :ThreadData = cv.clone();
    	println!("before _bv");
    }
    println!("after _bv");
    return;
}
