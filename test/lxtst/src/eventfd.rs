
use std::sync::Arc;
use std::cell::RefCell;
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use super::*;



struct EventFdInner {
	fd :i32,
}

extargs_error_class!{EventFdError}

impl Drop for EventFdInner {
	fn drop(&mut self) {
		self.close();
	}
}

#[allow(dead_code)]
impl EventFdInner {
	pub fn close(&mut self) {
		if self.fd >= 0 {
			unsafe {
				libc::close(self.fd);
			}
			self.fd = -1;
		}		
	}

	fn new(initval :i32) -> Result<Self,Box<dyn Error>> {
		let mut retv :Self = Self {
			fd : -1,
		};
		let cinitval :u32 = initval as u32;
		let cflags :libc::c_int = libc::EFD_NONBLOCK;
		let reti :i32;

		unsafe {
			retv.fd = libc::eventfd(cinitval,cflags);
		}
		if retv.fd < 0 {
			reti = get_errno!();
			extargs_new_error!{EventFdError,"can not open error {}",reti}
		}
		Ok(retv)
	}

	pub (crate) fn get_fd(&self) -> i32 {
		return self.fd;
	}

	pub  fn set_value(&self,val :i32) -> Result<(),Box<dyn Error>> {
		let mut reti :i32;
		let mut wval :libc::c_ulonglong = val as libc::c_ulonglong;

		if self.fd < 0 {
			extargs_new_error!{EventFdError,"not valid fd {}" ,self.fd}
		}

		unsafe {
			let _ptr = &mut wval as *mut libc::c_ulonglong as *mut libc::c_void;
			let wsize = std::mem::size_of::<libc::c_ulonglong>();
			reti = libc::write(self.fd,_ptr,wsize) as i32 ;
		}
		if reti < 0 {
			reti = get_errno!();
			extargs_new_error!{EventFdError,"write {} value error {}",val,reti}
		}
		Ok(())
	}

	pub fn get_value(&self) -> Result<i32,Box<dyn Error>> {
		let mut rval : libc::c_ulonglong = 0;
		let mut reti :i32;
		if self.fd < 0 {
			extargs_new_error!{EventFdError,"not valid fd {}" ,self.fd}
		}
		unsafe {
			let _ptr = &mut rval as *mut libc::c_ulonglong as *mut libc::c_void;
			let rsize = std::mem::size_of::<libc::c_ulonglong>();
			reti = libc::read(self.fd,_ptr,rsize) as i32;
		}
		if reti < 0 {
			reti = get_errno!();
			if reti == -libc::EINVAL {
				/*not valid*/
				return Ok(0);
			}
			extargs_new_error!{EventFdError,"read error {}",reti}
		}
		Ok(reti)
	}
}

pub struct EventFd {
	inner :Arc<RefCell<EventFdInner>>,
}

impl Drop for EventFd {
	fn drop(&mut self) {
		self.close();
	}
}


#[allow(dead_code)]
impl EventFd {
	pub fn close(&mut self) {

	}

	pub fn new(initval :i32) -> Result<Self,Box<dyn Error>> {
		let va = EventFdInner::new(initval)?;
		let retv :Self = Self {
			inner : Arc::new(RefCell::new(va)),
		};
		Ok(retv)
	}

	pub fn get_fd(&self) -> i32 {
		return self.inner.borrow().get_fd();
	}

	pub fn set_value(&self,val :i32) -> Result<(),Box<dyn Error>> {
		return self.inner.borrow().set_value(val);
	}

	pub fn get_value(&self) -> Result<i32,Box<dyn Error>> {
		return self.inner.borrow().get_value();
	}
}
