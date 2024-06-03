#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::io;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use sha2::{Sha256,Digest};


use std::error::Error;
use super::get_errno;

extargs_error_class!{FileOpError}

#[allow(dead_code)]
pub fn write_file_bytes(fname :&str, byts :&[u8]) -> Result<(),Box<dyn Error>> {
	if fname.len() == 0 {
		let res = io::stdout().write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [stdout] len[{}] error[{:?}]", byts.len(),err}	
		}
	} else {
		let fo  = fs::File::create(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"create [{}] error[{:?}]", fname,err}
		}
		let mut fp :fs::File = fo.unwrap();
		let res = fp.write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [{}] len[{}] error[{:?}]", fname, byts.len(),err}	
		}
	}
	Ok(())
}

#[allow(dead_code)]
pub fn read_file_bytes(fname :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	if fname.len() == 0 {
		let f = io::stdin();
		let mut reader = BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}
		Ok(buf)
	} else {
		let fo = fs::File::open(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"can not open [{}] error[{:?}]", fname, err}
		}
		let f = fo.unwrap();
		let mut reader = BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}

		Ok(buf)		
	}
}

#[allow(dead_code)]
pub fn read_file(fname :&str) -> Result<String,Box<dyn Error>> {
	if fname.len() == 0 {
		let f = io::stdin();
		let mut reader = BufReader::new(f);
		let mut retv :String = String::new();
		let res = reader.read_to_string(&mut retv);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}
		Ok(retv)
	} else {
		let fo = fs::File::open(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"can not open [{}] error[{:?}]", fname, err}
		}
		let f = fo.unwrap();
		let mut reader = BufReader::new(f);
		let mut retv :String = String::new();
		let res = reader.read_to_string(&mut retv);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}

		Ok(retv)		
	}
}

#[allow(dead_code)]
pub fn get_sha256_data(ind :&[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&ind);
    let res = hasher.finalize();
    return res.to_vec();    
}


pub struct FileFd {
	fd :i32,
	name :String,
}

impl FileFd {
	pub fn open(fname :&str,flags :libc::c_int) -> Result<Self,Box<dyn Error>> {
		let mut retv :Self = Self {
			fd :-1,
			name : format!("{}",fname),
		};
		let mut fstr :Vec<u8> = fname.as_bytes().to_vec();
		fstr.push(0);

		unsafe {
			let _fname = fstr.as_ptr() as *const i8;
			retv.fd = libc::open(_fname,flags);
		}

		if retv.fd < 0 {
			let reti = get_errno!();
			extargs_new_error!{FileOpError,"can not open {} error {}",fname,reti}
		}
		Ok(retv)
	}

	pub fn write(&self, bs :&[u8]) -> Result<isize,Box<dyn Error>> {
		let mut reti :isize;
		unsafe {
			let _ptr = bs.as_ptr() as * const libc::c_void;
			reti = libc::write(self.fd,_ptr,bs.len());
		}
		if reti < 0 {
			reti = get_errno!() as isize;
			extargs_new_error!{FileOpError,"write {} error {}",self.name,reti}
		}
		Ok(reti)
	}

}

impl Drop for FileFd {
	fn drop(&mut self) {
		if self.fd >= 0 {
			unsafe {
				libc::close(self.fd);
			}
			self.fd = -1;
		}
		return;
	}
}
