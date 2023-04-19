
use libc::{malloc,free,c_void};
use std::mem::{size_of};
use std::ptr::{null_mut};

pub struct AutoMem<T> {
	ptr :*mut T,
	vlen :usize,
}

#[allow(dead_code)]
impl<T : Sized>  AutoMem<T> {
	pub fn new(vlen :usize) -> Self {
		let mut tp :*mut T = null_mut();
		unsafe{
			if vlen != 0 {
				tp  = malloc(size_of::<T>() * vlen) as *mut T;	
				if tp == null_mut() {
					panic!("can not malloc 0x{:x} size", size_of::<T>() * vlen);
				}				
			}
		}
		
		AutoMem {
			ptr : tp,
			vlen : vlen,
		}
	}

	pub fn ptr(&self,idx :usize) -> *const T {
		let retp :*const T;
		if idx >= self.vlen {
			panic!("[{}] exceed size ", idx);
		}
		unsafe {
			retp = self.ptr.offset(idx as isize);
		}
		return retp;
	}

	pub fn ptr_mut(&mut self, idx :usize) -> *mut T {
		let retp :*mut T;
		if idx >= self.vlen {
			panic!("[{}] exceed size {}", idx,self.vlen);
		}

		unsafe {
			retp = self.ptr.offset(idx as isize);
		}
		return retp;
	}

	pub fn reset(&mut self,vlen :usize) {
		let mut tp :*mut T = null_mut();
		unsafe {
			if vlen != 0 {
				tp  = malloc(size_of::<T>() * vlen) as *mut T;	
				if tp == null_mut() {
					panic!("can not malloc 0x{:x} size", size_of::<T>() * vlen);
				}				
			}

			if self.ptr != null_mut() {
				free(self.ptr as *mut c_void);
			}
			self.ptr = null_mut();
		}
		self.ptr = tp;
		self.vlen = vlen;
	}

	pub fn size(&self) -> usize {
		return self.vlen;
	}
}

impl<T : Sized> Drop for AutoMem<T> {
	fn drop(&mut self) {
		if self.ptr != null_mut() {
			unsafe {
				free(self.ptr as *mut c_void);	
			}			
			self.ptr = null_mut();
		}
		self.vlen = 0;
	}
}

